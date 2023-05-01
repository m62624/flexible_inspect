use async_mutex::Mutex;
use pyo3::prelude::*;
use regex::Regex;
use std::sync::Arc;
#[derive(Debug, Clone)]
pub struct PythonSafeObject {
    original_class: Arc<Mutex<PyObject>>,
    regex_collection: Vec<Regex>,
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Validator {
    inner: Vec<PythonSafeObject>,
}
