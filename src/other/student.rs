use pyo3::{pyclass, pymethods};

#[pyclass]
pub struct Student {
    name: String,
    age: i16,
}


#[pymethods]
impl Student {
    #[new]
    fn new() -> Self {
        Student {
            name: "张三".into(),
            age: 18,
        }
    }
}