use pyo3::{create_exception, exceptions::PyException};

// 定义父异常类
create_exception!(rust_python, MyError, PyException);

// 定义异常子类
create_exception!(rust_python, ChildErrorA, MyError);
create_exception!(rust_python, ChildErrorB, MyError);
create_exception!(rust_python, ChildErrorC, MyError);

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("An IO error occurred")]
    Io(#[from] std::io::Error),
    #[error("An ODS error occurred")]
    Ods,
    #[error("An XLS error occurred")]
    Xls,
    #[error("validation error: {0}")]
    ValidationError(String),
}

impl From<Error> for pyo3::PyErr {
    fn from(err: Error) -> Self {
        match err {
            Error::Io(e) => MyError::new_err(format!("IO error: {}", e)),
            Error::Ods => ChildErrorA::new_err(err.to_string()),
            Error::Xls => ChildErrorB::new_err(err.to_string()),
            Error::ValidationError(_) => pyo3::exceptions::PyValueError::new_err(err.to_string()),
        }
    }
}
