use pyo3::{*, types::{PyDict, PyTuple}};
use std::collections::HashMap;
use crate::exception::*;

/// 默认参数以及不定长参数
#[pyfunction]
#[pyo3(signature = (num = 10, * py_args, * * py_kwargs))]
pub fn many_args(num: i32, py_args: &Bound<'_, PyTuple>, py_kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<String> {
    println!("rust function many_args start...");
    let result = format!(
        "func many_args => num: {}  py_args: {:?} py_kwargs: {:?} ",
        num, py_args, py_kwargs,
    );
    Ok(result)
}


/// 函数的参数可以是rust类型，自动转换，失败会报错
#[pyfunction]
pub fn dic_to_list(input_dic: HashMap<String, String>) -> PyResult<Vec<String>> {
    println!("rust function dic_to_list start...");

    let mut result = Vec::new();
    for (k, v) in input_dic {
        println!("遍历字典: {k}: {v}");
        result.push(v)
    }
    Ok(result)
}

/// 列表转字典
#[pyfunction]
pub fn list_to_dic(names: Vec<String>) -> PyResult<HashMap<usize, String>> {
    println!("rust function list_to_dic start...");

    let mut result = HashMap::new();
    for (index, value) in names.iter().enumerate() {
        println!("遍历列表: {:?}", value);
        result.insert(index, value.to_owned());
    }
    Ok(result)
}


/// 类实例作为型参  操作对象的不可变借用
#[pyfunction]
pub fn student_info(stu: &Student) -> String {
    println!("rust function student_info start...");
    stu.get_info()
}

/// 类实例作为型参  操作对象的可变借用
#[pyfunction]
pub fn student_set_age(stu: &mut Student, age: u32) {
    println!("rust function student_set_age start...");
    stu.set_age(age);
}


/// 这个是类的描述
#[pyclass(get_all)]
pub struct Student {
    name: String,
    age: u32,
}


/// Student 暴露给python调用的方法
#[pymethods]
impl Student {
    /// 打印对象的时候调用
    fn __repr__(&self) -> String {
        format!(
            "Student(name='{}', age={})",
            self.name, self.age
        )
    }

    #[new]
    pub fn py_new(name: String, age: u32) -> Self {
        Student { name, age }
    }

    /// 抛出自定义异常
    pub fn raise_exception(&self, number: Option<i32>) -> PyResult<String> {
        println!("rust function raise_exception start...");
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

    pub fn py_set_large_age(&mut self, ages: Vec<u32>) -> PyResult<u32> {
        println!("rust function py_set_large_age start...");
        let age = ages.iter().max();
        self.age = age.unwrap().to_owned();
        Ok(self.age)
    }
}

// Student 编写与python无关的方法
impl Student {
    // 不可变借用的方法
    pub fn get_info(&self) -> String {
        format!("Name: {}, Age: {}", self.name, self.age)
    }

    // 可变借用的方法
    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}