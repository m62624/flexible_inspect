use pyo3::prelude::*;
use std::collections::HashMap;
mod check_convert;
mod init;
mod match_requirement;
mod rule;
#[pyclass]
#[derive(Debug, Clone)]
pub struct TemplateValidator {
    #[pyo3(get, set)]
    py_classes: HashMap<usize, PyObject>,
    #[pyo3(get, set)]
    rules: HashMap<rule::Rule, usize>,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ContainerTree {
    selected_rules: regex::RegexSet,
}
