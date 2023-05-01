use async_mutex::Mutex;
use pyo3::prelude::*;
use regex::Regex;
use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct PythonSafeObject {
    original_class: PyObject,
    regex_collection: Vec<Regex>,
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Validator {
    inner: Arc<Mutex<Vec<PythonSafeObject>>>,
}
mod init_validator {
    use super::*;
    #[pymethods]
    impl Validator {
        #[new]
        fn py_constructor() -> Self {
            todo!()
        }
        /// минимальный набор методов для отладки
        fn __repr__(&self) -> PyResult<String> {
            Ok(format!("Validator({:#?})", self.inner))
        }
        fn __str__(&self) -> String {
            format!("{:#?}", self.inner)
        }
    }
    impl Validator {}
}
