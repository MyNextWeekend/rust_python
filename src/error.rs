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
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("权限不足")]
    Unauthorized,
    #[error("非法状态: {0}")]
    InvalidState(String),
    #[error("validation error: {0}")]
    InvalidParameter(String),
}

// 转换为 自定义的 python 异常
impl From<Error> for pyo3::PyErr {
    fn from(err: Error) -> Self {
        match err {
            Error::Io(e) => MyError::new_err(e.to_string()),
            Error::Unauthorized => ChildErrorA::new_err(err.to_string()),
            Error::InvalidState(_) => ChildErrorB::new_err(err.to_string()),
            Error::InvalidParameter(_) => pyo3::exceptions::PyValueError::new_err(err.to_string()),
        }
    }
}
