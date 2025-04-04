use pyo3::prelude::*;

mod example;
mod error;

pub use crate::example::*;
pub use crate::error::{ChildErrorA, ChildErrorB, ChildErrorC, MyError};


/// 这个是模块描述：在Rust中实现的Python模块。
#[pymodule]
mod _core {
    use super::*;

    #[pymodule_export]
    use crate::{
        many_args, dic_to_list, list_to_dic, parallel_sum_of_squares,
        student_info, student_set_age, Student,
        MyError, ChildErrorA, ChildErrorB, ChildErrorC,
    };
}

// /// 这个是模块描述：在Rust中实现的Python模块。
// #[pymodule]
// fn _lowlevel(py: Python,m: &Bound<'_, PyModule>) -> PyResult<()> {
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

