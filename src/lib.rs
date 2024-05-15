use pyo3::prelude::*;

mod other;

use other::student::Student;

/// rust 方法
#[pyfunction]
fn dic_to_list() -> PyResult<String> {
    Ok("Hello from dic_to_list!".into())
}

#[pyfunction]
fn list_to_dic() -> PyResult<String> {
    Ok("Hello from list_to_dic!".into())
}


/// 在Rust中实现的Python模块。
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dic_to_list, m)?)?;
    m.add_function(wrap_pyfunction!(list_to_dic, m)?)?;
    m.add_class::<Student>()?;
    Ok(())
}
