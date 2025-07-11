use log::info;
use pyo3::{
    types::{PyDict, PyTuple},
    *,
};
use std::collections::HashMap;

/// 默认参数以及不定长参数
#[pyfunction]
#[pyo3(signature = (num = 10, * py_args, * * py_kwargs))]
pub fn many_args(
    num: i32,
    py_args: &Bound<'_, PyTuple>,
    py_kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    info!("rust function many_args start...");
    let result = format!(
        "func many_args => num: {}  py_args: {:?} py_kwargs: {:?} ",
        num, py_args, py_kwargs,
    );
    Ok(result)
}

/// 函数的参数可以是rust类型，自动转换，失败会报错
#[pyfunction]
pub fn dic_to_list(input_dic: HashMap<String, String>) -> PyResult<Vec<String>> {
    info!("rust function dic_to_list start...");

    let mut result = Vec::new();
    for (_, v) in input_dic {
        // println!("遍历字典: {k}: {v}");
        result.push(v)
    }
    Ok(result)
}

/// 列表转字典
#[pyfunction]
pub fn list_to_dic(names: Vec<String>) -> PyResult<HashMap<usize, String>> {
    info!("rust function list_to_dic start...");

    let mut result = HashMap::new();
    for (index, value) in names.iter().enumerate() {
        // println!("遍历列表: {:?}", value);
        result.insert(index, value.to_owned());
    }
    Ok(result)
}
