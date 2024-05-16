use std::collections::HashMap;

use pyo3::{prelude::*, types::{PyDict, PyTuple, PyList}};

mod other;

use other::student::Student;

// 默认参数以及不定长参数
#[pyfunction]
#[pyo3(signature = (num = 10, * py_args, * * py_kwargs))]
fn many_args(num: i32, py_args: &PyTuple, py_kwargs: Option<&PyDict>) -> PyResult<String> {
    println!("rust function many_args start");
    let result = format!(
        "num: {}  py_args: {:?} py_kwargs: {:?} ",
        num, py_args, py_kwargs,
    );
    Ok(result)
}


#[pyfunction]
#[pyo3(signature = (input_dic))]
fn dic_to_list(input_dic: &PyDict) -> PyResult<Vec<&PyAny>> {
    println!("rust function dic_to_list start");

    let mut result = Vec::new();
    for (k, v) in input_dic {
        println!("遍历字典: {k}: {v}");
        result.push(v)
    }
    Ok(result)
}

#[pyfunction]
#[pyo3(signature = (names))]
fn list_to_dic(names: &PyList) -> PyResult<HashMap<usize, &PyAny>> {
    println!("rust function list_to_dic start");

    let mut result = HashMap::new();
    for (index, value) in names.iter().enumerate() {
        println!("遍历列表: {:?}", value);
        result.insert(index, value);
    }
    Ok(result)
}


/// 在Rust中实现的Python模块。
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(many_args, m)?)?;
    m.add_function(wrap_pyfunction!(dic_to_list, m)?)?;
    m.add_function(wrap_pyfunction!(list_to_dic, m)?)?;
    m.add_class::<Student>()?;
    Ok(())
}
