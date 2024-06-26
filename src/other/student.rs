use pyo3::{pyclass, pymethods, PyResult};

use super::{ChildErrorA, ChildErrorB, ChildErrorC, MyError};


#[pyclass(get_all)]
pub struct Student {
    name: String,
    age: i32,
}


/// Student 暴露给python调用的方法
#[pymethods]
impl Student {
    fn __repr__(&self) -> String {
        format!(
            "Student(name='{}', age={})",
            self.name, self.age
        )
    }

    #[new]
    fn py_new(name: String, age: i32) -> Self {
        Student {
            name: name,
            age: age,
        }
    }

    /// 抛出自定义异常
    fn raise_exception(&self, number: Option<i32>) -> PyResult<String> {
        match number {
            Some(0) => {
                Err(MyError::new_err("MyError".to_string()))
            }
            Some(1) => {
                Err(ChildErrorA::new_err("A_ERR".to_string()))
            }
            Some(2) => {
                Err(ChildErrorB::new_err("B_ERR".to_string()))
            }
            Some(3) => {
                Err(ChildErrorC::new_err("C_ERR".to_string()))
            }
            _ => {
                Ok("ok".into())
            }
        }
    }

    fn py_set_lages_age(&mut self,ages: Vec<i32>) -> PyResult<i32> {
        let age = ages.iter().max();
        self.age= age.unwrap().to_owned();
        Ok(self.age)
    }

}

// Student 编写与python无关的方法
impl Student {

}