use pyo3::{
    types::{PyDict, PyTuple},
    *,
};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// 默认参数以及不定长参数
#[pyfunction]
#[pyo3(signature = (num = 10, * py_args, * * py_kwargs))]
pub fn many_args(
    num: i32,
    py_args: &Bound<'_, PyTuple>,
    py_kwargs: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    log::info!("rust function many_args start...");

    let result = format!(
        "func many_args => num: {}  py_args: {:?} py_kwargs: {:?} ",
        num, py_args, py_kwargs,
    );
    Ok(result)
}

/// 函数的参数可以是rust类型，自动转换，失败会报错
#[pyfunction]
pub fn dic_to_list(input_dic: HashMap<String, String>) -> PyResult<Vec<String>> {
    log::info!("rust function dic_to_list start...");

    let mut result = Vec::new();
    for (_, v) in input_dic {
        result.push(v)
    }
    Ok(result)
}

/// 列表转字典
#[pyfunction]
pub fn list_to_dic(names: Vec<String>) -> PyResult<HashMap<usize, String>> {
    log::info!("rust function list_to_dic start...");

    let mut result = HashMap::new();
    for (index, value) in names.iter().enumerate() {
        result.insert(index, value.to_owned());
    }
    Ok(result)
}

/// 字符串处理：反转字符串
#[pyfunction]
pub fn reverse_string(s: &str) -> PyResult<String> {
    log::info!("rust function reverse_string start...");

    Ok(s.chars().rev().collect())
}

/// 字符串处理：计算字符串哈希
#[pyfunction]
pub fn hash_string(s: &str) -> PyResult<u64> {
    log::info!("rust function hash_string start...");

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut hasher);
    Ok(hasher.finish())
}

/// 数值计算：计算阶乘
#[pyfunction]
pub fn factorial(n: u64) -> PyResult<u64> {
    log::info!("rust function factorial start...");

    if n > 20 {
        return Err(pyo3::exceptions::PyOverflowError::new_err(
            "Factorial exceeds u64 range for n > 20",
        ));
    }

    let mut result = 1u64;
    for i in 2..=n {
        result *= i;
    }
    Ok(result)
}

/// 数值计算：斐波那契数列（迭代方式）
#[pyfunction]
pub fn fibonacci(n: u32) -> PyResult<u64> {
    log::info!("rust function fibonacci start...");

    if n == 0 {
        return Ok(0);
    } else if n == 1 {
        return Ok(1);
    }

    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    Ok(b)
}

/// 数组处理：数组去重并排序
#[pyfunction]
pub fn unique_sorted(arr: Vec<i64>) -> PyResult<Vec<i64>> {
    log::info!("rust function unique_sorted start...");

    let mut arr = arr;
    arr.sort_unstable();
    arr.dedup();
    Ok(arr)
}

/// 数组处理：数组分块
#[pyfunction]
pub fn chunk_array(arr: Vec<i64>, chunk_size: usize) -> PyResult<Vec<Vec<i64>>> {
    log::info!("rust function chunk_array start...");

    if chunk_size == 0 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Chunk size must be greater than 0",
        ));
    }

    let chunks: Vec<Vec<i64>> = arr.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();
    Ok(chunks)
}

/// 数组处理：扁平化嵌套数组（简单版本）
#[pyfunction]
pub fn flatten_array(arrays: Vec<Vec<i64>>) -> PyResult<Vec<i64>> {
    log::info!("rust function flatten_array start...");

    let flattened: Vec<i64> = arrays.into_iter().flatten().collect();
    Ok(flattened)
}

/// 性能测试：生成大量数据
#[pyfunction]
pub fn generate_large_array(size: usize) -> PyResult<Vec<i64>> {
    log::info!("rust function generate_large_array start...");

    let result: Vec<i64> = (0..size as i64).collect();
    Ok(result)
}
