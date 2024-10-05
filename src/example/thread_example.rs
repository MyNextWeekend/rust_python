use pyo3::{pyfunction, PyResult, Python};
use std::thread;
use std::time::Duration;

/// 利用多线程实现真实的并发
#[pyfunction]
pub fn parallel_sum_of_squares(num: usize) -> PyResult<u128> {
    // 释放 GIL 以便 Rust 能够利用所有 CPU 核心
    Python::with_gil(|py| py.allow_threads(|| {
        thread::sleep(Duration::from_secs(3));
        let mut sum: u128 = 0;
        for i in 0..num + 1 {
            let i1 = i as u128;
            sum += i1 * i1;
        }
        Ok(sum)
    }))
}