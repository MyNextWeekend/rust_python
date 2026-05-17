use pyo3::{pyfunction, PyResult, Python};
use rayon::prelude::*;

/// 使用 Rayon 实现真正的并行计算
/// 计算从 0 到 num 的所有整数的平方和
#[pyfunction]
pub fn parallel_sum_of_squares(py: Python<'_>, num: usize) -> PyResult<u128> {
    py.allow_threads(|| {
        let sum: u128 = (0..=num).into_par_iter().map(|i| {
            let i1 = i as u128;
            i1 * i1
        }).sum();
        Ok(sum)
    })
}

/// 并行计算数组中每个元素的平方
#[pyfunction]
pub fn parallel_square_array(py: Python<'_>, arr: Vec<i64>) -> PyResult<Vec<i64>> {
    py.allow_threads(|| {
        let result: Vec<i64> = arr.into_par_iter()
            .map(|x| x * x)
            .collect();
        Ok(result)
    })
}

/// 并行计算数组的总和
#[pyfunction]
pub fn parallel_sum_array(py: Python<'_>, arr: Vec<i64>) -> PyResult<i64> {
    py.allow_threads(|| {
        let sum: i64 = arr.into_par_iter().sum();
        Ok(sum)
    })
}

/// 并行过滤数组中的偶数
#[pyfunction]
pub fn parallel_filter_even(py: Python<'_>, arr: Vec<i64>) -> PyResult<Vec<i64>> {
    py.allow_threads(|| {
        let result: Vec<i64> = arr.into_par_iter()
            .filter(|&x| x % 2 == 0)
            .collect();
        Ok(result)
    })
}

/// 并行归约：计算数组中的最大值
#[pyfunction]
pub fn parallel_max(py: Python<'_>, arr: Vec<i64>) -> PyResult<Option<i64>> {
    py.allow_threads(|| {
        let max = arr.into_par_iter().max();
        Ok(max)
    })
}

/// 并行映射归约：计算数组的平均值
#[pyfunction]
pub fn parallel_average(py: Python<'_>, arr: Vec<f64>) -> PyResult<f64> {
    py.allow_threads(|| {
        let (sum, count) = arr.into_par_iter()
            .map(|x| (x, 1usize))
            .reduce(|| (0.0, 0), |(a_sum, a_cnt), (b_sum, b_cnt)| (a_sum + b_sum, a_cnt + b_cnt));
        
        if count == 0 {
            Ok(0.0)
        } else {
            Ok(sum / count as f64)
        }
    })
}

/// 使用 Rayon 的线程池进行并行计算
/// 设置线程池大小后执行并行任务
#[pyfunction]
pub fn parallel_with_thread_pool(py: Python<'_>, num: usize, threads: usize) -> PyResult<u128> {
    py.allow_threads(|| {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create thread pool: {}", e)))?;
        
        let sum: u128 = pool.install(|| {
            (0..=num).into_par_iter().map(|i| {
                let i1 = i as u128;
                i1 * i1
            }).sum()
        });
        
        Ok(sum)
    })
}