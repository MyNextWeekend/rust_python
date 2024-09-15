use std::collections::HashMap;

use pyo3::{prelude::*, types::{PyDict, PyTuple}};

mod other;

use other::{student::Student, ChildErrorA, ChildErrorB, ChildErrorC, MyError};

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


// 函数的参数可以是rust类型，自动转换，失败会报错
#[pyfunction]
#[pyo3(signature = (input_dic))]
fn dic_to_list(input_dic: HashMap<String, String>) -> PyResult<Vec<String>> {
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
fn list_to_dic(names: Vec<String>) -> PyResult<HashMap<usize, String>> {
    println!("rust function list_to_dic start");

    let mut result = HashMap::new();
    for (index, value) in names.iter().enumerate() {
        println!("遍历列表: {:?}", value);
        result.insert(index, value.to_owned());
    }
    Ok(result)
}

/// 利用多线程实现真实的并发
#[pyfunction]
fn parallel_sum_of_squares(num: usize) -> PyResult<u128> {
    // 释放 GIL 以便 Rust 能够利用所有 CPU 核心
    Python::with_gil(|py| py.allow_threads(|| {
        let mut sum :u128 = 0;
        for i in 0..num+1 {
            let i1 = i as u128;
            sum += i1 * i1;
        }
        Ok(sum)
    }))
}

/// 在Rust中实现的Python模块。
#[pymodule]
fn _lowlevel(py: Python, m: &PyModule) -> PyResult<()> {
    // 添加函数
    m.add_function(wrap_pyfunction!(many_args, m)?)?;
    m.add_function(wrap_pyfunction!(dic_to_list, m)?)?;
    m.add_function(wrap_pyfunction!(list_to_dic, m)?)?;
    m.add_function(wrap_pyfunction!(parallel_sum_of_squares, m)?)?;
    // 添加类
    m.add_class::<Student>()?;
    // 添加异常
    m.add("MyError", py.get_type::<MyError>())?;
    m.add("ChildErrorA", py.get_type::<ChildErrorA>())?;
    m.add("ChildErrorB", py.get_type::<ChildErrorB>())?;
    m.add("ChildErrorC", py.get_type::<ChildErrorC>())?;
    Ok(())
}
