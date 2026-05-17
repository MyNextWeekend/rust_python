use crate::error::{Error, Result};
use pyo3::{types::PyType, *};

/// 类实例作为型参  操作对象的不可变借用
#[pyfunction]
pub fn student_info(stu: &Student) -> String {
    log::info!("rust function student_info start...");

    stu.get_info()
}

/// 类实例作为型参  操作对象的可变借用
#[pyfunction]
pub fn student_set_age(stu: &mut Student, age: u32) -> PyResult<()> {
    log::info!("rust function student_set_age start...");

    stu.set_age(age)?;
    Ok(())
}

/// 学生类
#[pyclass(get_all, set_all)]
#[derive(Clone)]
pub struct Student {
    name: String,
    age: u32,
    grade: Option<String>,
    gpa: Option<f32>,
}

/// Student 暴露给 Python 调用的方法
#[pymethods]
impl Student {
    fn __repr__(&self) -> String {
        format!(
            "Student(name='{}', age={}, grade={:?}, gpa={:?})",
            self.name, self.age, self.grade, self.gpa
        )
    }

    #[classmethod]
    fn from_default(_cls: &Bound<'_, PyType>) -> Self {
        Self::default_impl()
    }

    #[classmethod]
    fn from_dict(_cls: &Bound<'_, PyType>, data: std::collections::HashMap<String, String>) -> PyResult<Self> {
        let name = data.get("name").cloned().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Missing 'name' key")
        })?;
        let age: u32 = data.get("age")
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Missing 'age' key"))?
            .parse()
            .map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid 'age' value"))?;
        
        let grade = data.get("grade").cloned();
        let gpa: Option<f32> = data.get("gpa")
            .map(|s| s.parse().map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid 'gpa' value")))
            .transpose()?;
        
        Ok(Self { name, age, grade, gpa })
    }

    #[new]
    #[pyo3(signature = (name, age, grade = None, gpa = None))]
    fn new(name: String, age: u32, grade: Option<String>, gpa: Option<f32>) -> Self {
        Student { name, age, grade, gpa }
    }

    fn raise_exception(&self, number: Option<i32>) -> PyResult<String> {
        Ok(self.raise_exception_impl(number)?)
    }

    fn set_large_age(&mut self, ages: Vec<u32>) -> PyResult<u32> {
        log::info!("rust function set_large_age start...");

        let age = ages.iter().max();
        if let Some(&age) = age {
            self.age = age;
            Ok(age)
        } else {
            Err(Error::InvalidParameter("输入的列表为空".to_string()).into())
        }
    }

    fn set_other_age(&mut self, stu: &Student) -> PyResult<()> {
        Ok(self.set_age(stu.age)?)
    }

    fn calculate_birth_year(&self) -> u32 {
        2024 - self.age
    }

    #[pyo3(signature = (name = None, age = None))]
    fn update_info(&mut self, name: Option<String>, age: Option<u32>) -> PyResult<()> {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(age) = age {
            self.set_age(age)?;
        }
        Ok(())
    }

    fn get_full_description(&self) -> String {
        let mut desc = format!("{}，{}岁", self.name, self.age);
        if let Some(grade) = &self.grade {
            desc.push_str(&format!("，年级：{}", grade));
        }
        if let Some(gpa) = self.gpa {
            desc.push_str(&format!("，GPA：{:.2}", gpa));
        }
        desc
    }
}

impl Student {
    fn default_impl() -> Self {
        log::info!("rust function default_impl start...");

        Self {
            name: "Default Student".to_string(),
            age: 18,
            grade: None,
            gpa: None,
        }
    }

    fn get_info(&self) -> String {
        format!("Name: {}, Age: {}", self.name, self.age)
    }

    fn set_age(&mut self, age: u32) -> Result<()> {
        if age == 0 || age > 120 {
            return Err(Error::InvalidParameter(format!("age 需要在1-120之间，当前值: {}", age)));
        }
        self.age = age;
        Ok(())
    }

    fn raise_exception_impl(&self, number: Option<i32>) -> Result<String> {
        log::info!("rust function raise_exception_impl start...");

        match number {
            Some(n) if n < 0 => Err(Error::Unauthorized),
            Some(n) if n > 100 => Err(Error::InvalidState(n.to_string())),
            _ => Ok(format!("No exception raised, number: {:?}", number)),
        }
    }
}

/// 教师类
#[pyclass(get_all, set_all)]
pub struct Teacher {
    name: String,
    subject: String,
    students: Vec<Student>,
}

#[pymethods]
impl Teacher {
    #[new]
    fn new(name: String, subject: String) -> Self {
        Teacher {
            name,
            subject,
            students: Vec::new(),
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Teacher(name='{}', subject='{}', students={})",
            self.name, self.subject, self.students.len()
        )
    }

    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    fn get_student_count(&self) -> usize {
        self.students.len()
    }

    fn get_all_students_info(&self) -> Vec<String> {
        self.students.iter().map(|s| s.get_info()).collect()
    }

    fn get_average_age(&self) -> f64 {
        if self.students.is_empty() {
            0.0
        } else {
            let sum: u32 = self.students.iter().map(|s| s.age).sum();
            sum as f64 / self.students.len() as f64
        }
    }
}