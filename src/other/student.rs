use pyo3::{pyclass, pymethods, types::{PyLong, PyUnicode}, PyResult};

#[pyclass]
pub struct Student {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    age: i32,
}


#[pymethods]
impl Student {
    fn __repr__(&self) -> String {
        format!(
            "Student(name='{}', age={})",
            self.name, self.age
        )
    }

    #[new]
    fn py_new(name:String,age:i32) -> Self {
        Student {
            name: name,
            age: age,
        }
    }
}

impl Student {
    
}