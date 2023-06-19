use pyo3::prelude::*;
use std::collections::HashMap;
mod check_convert;
mod contrainer_tree;
mod init;
mod match_requirement;
mod rule;
#[pyclass]
#[derive(Debug, Clone)]
pub struct TemplateValidator {
    #[pyo3(get, set)]
    py_classes: HashMap<usize, PyObject>,
    isolated_environment: contrainer_tree::ContainerTree,
}
