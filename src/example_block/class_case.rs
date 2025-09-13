use crate::error::{Error, Result};
use log::info;
use pyo3::{types::PyType, *};

/// 类实例作为型参  操作对象的不可变借用
#[pyfunction]
pub fn student_info(stu: &Student) -> String {
    info!("rust function student_info start...");
    stu.get_info()
}

/// 类实例作为型参  操作对象的可变借用
#[pyfunction]
pub fn student_set_age(stu: &mut Student, age: u32) -> PyResult<()> {
    info!("rust function student_set_age start...");
    stu.set_age(age)?;
    Ok(())
}

/// 这个是类的描述
#[pyclass(get_all, set_all)]
pub struct Student {
    name: String,
    age: u32,
}

/// Student 暴露给python调用的方法
#[pymethods]
impl Student {
    /// 打印对象的时候调用
    fn __repr__(&self) -> String {
        format!("Student(name='{}', age={})", self.name, self.age)
    }

    /// 类函数
    #[classmethod]
    #[pyo3(name = "from_xx")]
    fn py_from_xx(_cls: &Bound<'_, PyType>, _py: Python<'_>) -> PyResult<Self> {
        Ok(Self::from_xx()?)
    }

    #[new]
    fn py_new(name: String, age: u32) -> Self {
        Student { name, age }
    }

    /// 抛出自定义异常
    #[pyo3(name = "raise_exception")]
    fn py_raise_exception(&self, number: Option<i32>) -> PyResult<String> {
        Ok(self.raise_exception(number)?)
    }

    #[pyo3(name = "set_large_age")]
    fn py_set_large_age(&mut self, ages: Vec<u32>) -> PyResult<u32> {
        info!("rust function py_set_large_age start...");
        let age = ages.iter().max();
        self.age = age.unwrap().to_owned();
        Ok(self.age)
    }

    #[pyo3(name = "set_other_age")]
    fn py_set_other_age(&self, stu: &mut Student) {
        self.set_other_age(stu);
    }
}

// Student 编写与python无关的方法
impl Student {
    fn from_xx() -> Result<Self> {
        info!("rust function from_filelike start...");
        Ok(Self {
            name: "Default Student".to_string(),
            age: 18,
        })
    }

    // 不可变借用的方法
    fn get_info(&self) -> String {
        format!("Name: {}, Age: {}", self.name, self.age)
    }

    // 可变借用的方法
    fn set_age(&mut self, age: u32) -> Result<()> {
        if age <= 0 || age > 120 {
            return Err(Error::ValidationError(age.to_string()));
        }
        self.age = age;
        Ok(())
    }

    fn set_other_age(&self, stu: &mut Student) {
        let _ = stu.set_age(self.age).unwrap();
    }

    fn raise_exception(&self, number: Option<i32>) -> Result<String> {
        info!("rust function raise_exception start...");
        match number {
            Some(n) if n < 0 => Err(Error::ValidationError("不能小于0".to_string())),
            Some(n) if n > 100 => Err(Error::ValidationError("不能大于100".to_string())),
            _ => Ok(format!("No exception raised, number: {:?}", number)),
        }
    }
}
