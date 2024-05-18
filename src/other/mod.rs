use pyo3::create_exception;

pub mod student;


// 定义父异常类
create_exception!(rust_python, MyError, pyo3::exceptions::PyException);

// 定义异常子类
create_exception!(rust_python, ChildErrorA, MyError);
create_exception!(rust_python, ChildErrorB, MyError);
create_exception!(rust_python, ChildErrorC, MyError);