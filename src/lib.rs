mod error;
mod example_block;
mod example_thread;

/// 在 Rust 中实现的 Python 核心模块。
///
/// 该模块通过 PyO3 绑定提供高性能的数据处理函数
#[pyo3::pymodule]
mod _core {

    use pyo3::{types::PyModule, Bound, PyResult};

    #[pymodule_export]
    use crate::example_block::{
        dic_to_list, list_to_dic, many_args, student_info, student_set_age, Student,
    };

    #[pymodule_export]
    use crate::example_thread::parallel_sum_of_squares;

    #[pymodule_export]
    use crate::error::{ChildErrorA, ChildErrorB, ChildErrorC, MyError};

    /// 在模块初始化时 运行
    /// 在 Python 执行 `import` 时触发，用于初始化日志系统。
    #[pymodule_init]
    fn init(_m: &Bound<'_, PyModule>) -> PyResult<()> {
        // 初始化 pyo3-log，允许在 Rust 中使用 log 宏并将输出重定向到 Python
        pyo3_log::init();
        Ok(())
    }
}

// /// 这个是模块描述：在Rust中实现的Python模块。
// #[pymodule]
// fn _core(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
//     // A good place to install the Rust -> Python logger.
//     pyo3_log::init();
//     // 添加函数
//     m.add_function(wrap_pyfunction!(many_args, m)?)?;
//     m.add_function(wrap_pyfunction!(dic_to_list, m)?)?;
//     m.add_function(wrap_pyfunction!(list_to_dic, m)?)?;
//     m.add_function(wrap_pyfunction!(parallel_sum_of_squares, m)?)?;
//     // 方法型参是类实例
//     m.add_function(wrap_pyfunction!(student_info, m)?)?;
//     m.add_function(wrap_pyfunction!(student_set_age, m)?)?;
//     // 添加类
//     m.add_class::<Student>()?;
//     // 添加异常
//     m.add("MyError", py.get_type::<MyError>())?;
//     m.add("ChildErrorA", py.get_type::<ChildErrorA>())?;
//     m.add("ChildErrorB", py.get_type::<ChildErrorB>())?;
//     m.add("ChildErrorC", py.get_type::<ChildErrorC>())?;
//     Ok(())
// }
