use pyo3::{pyfunction, PyResult, Python};

/// 利用多线程实现真实的并发
#[pyfunction]
pub fn parallel_sum_of_squares(py: Python<'_>, num: usize) -> PyResult<u128> {
    py.allow_threads(|| {
        let mut sum: u128 = 0;
        for i in 0..num + 1 {
            let i1 = i as u128;
            sum += i1 * i1;
        }
        Ok(sum)
    })
}
