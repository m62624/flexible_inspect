mod step_by_step;
use pyo3::exceptions;

use super::rule::Rule;
use super::*;
mod getters;
#[derive(Debug)]
/// --> TemplateValidator
pub struct ExceptionContainer {
    py_class: PyObject,
    default_r: Vec<Rule>,
    fancy_r: Vec<Rule>,
}
impl ExceptionContainer {
    pub fn new(py: Python, py_class: PyObject) -> PyResult<Self> {
        if let Ok(class_py) = py_class.downcast::<types::PyType>(py) {
            let mut default_r = Vec::new();
            let mut fancy_r = Vec::new();
            Self::get_all_rules_from_class(class_py, &mut default_r, &mut fancy_r)?;
            return Ok(Self {
                py_class,
                default_r,
                fancy_r,
            });
        } else {
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'Class'",
                py_class
            )));
        }
    }
}
