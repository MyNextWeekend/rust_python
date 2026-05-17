use pyo3::{create_exception, exceptions::PyException};

// 定义父异常类
create_exception!(rust_python, MyError, PyException);

// 定义异常子类
create_exception!(rust_python, ChildErrorA, MyError);
create_exception!(rust_python, ChildErrorB, MyError);
create_exception!(rust_python, ChildErrorC, MyError);
create_exception!(rust_python, ValidationError, MyError);
create_exception!(rust_python, DatabaseError, MyError);
create_exception!(rust_python, NetworkError, MyError);

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("权限不足")]
    Unauthorized,

    #[error("非法状态: {0}")]
    InvalidState(String),

    #[error("参数验证失败: {0}")]
    InvalidParameter(String),

    #[error("数据库错误: {0}")]
    Database(String),

    #[error("网络错误: {0}")]
    Network(String),

    #[error("索引越界: 索引 {index} 超出范围 [0, {max}]")]
    IndexOutOfBounds { index: usize, max: usize },

    #[error("空值错误: {0}")]
    NullValue(String),

    #[error("转换错误: {0}")]
    Conversion(String),

    #[error("配置错误: {0}")]
    Config(String),

    #[error("资源未找到: {0}")]
    NotFound(String),

    #[error("重复数据: {0}")]
    Duplicate(String),

    #[error("超时错误: {0}")]
    Timeout(String),

    #[error("未知错误: {0}")]
    Unknown(String),
}

// 转换为自定义的 Python 异常
impl From<Error> for pyo3::PyErr {
    fn from(err: Error) -> Self {
        match err {
            Error::Io(e) => MyError::new_err(e.to_string()),
            Error::Unauthorized => ChildErrorA::new_err(err.to_string()),
            Error::InvalidState(_) => ChildErrorB::new_err(err.to_string()),
            Error::InvalidParameter(msg) => ValidationError::new_err(msg),
            Error::Database(msg) => DatabaseError::new_err(msg),
            Error::Network(msg) => NetworkError::new_err(msg),
            Error::IndexOutOfBounds { .. } => {
                pyo3::exceptions::PyIndexError::new_err(err.to_string())
            }
            Error::NullValue(msg) => pyo3::exceptions::PyValueError::new_err(msg),
            Error::Conversion(msg) => pyo3::exceptions::PyTypeError::new_err(msg),
            Error::Config(msg) => MyError::new_err(msg),
            Error::NotFound(msg) => pyo3::exceptions::PyKeyError::new_err(msg),
            Error::Duplicate(msg) => pyo3::exceptions::PyRuntimeError::new_err(msg),
            Error::Timeout(msg) => pyo3::exceptions::PyTimeoutError::new_err(msg),
            Error::Unknown(msg) => MyError::new_err(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let err = Error::IndexOutOfBounds { index: 10, max: 5 };
        assert!(err.to_string().contains("10"));
        assert!(err.to_string().contains("5"));

        let err = Error::NotFound("user".to_string());
        assert!(err.to_string().contains("user"));
    }
}
