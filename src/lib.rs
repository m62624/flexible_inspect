use pyo3::{prelude::*, types::PyType};
use regex::Regex;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PythonSafeObject {
    original_class: Arc<PyType>,
    regex_collection: Vec<Regex>,
}
