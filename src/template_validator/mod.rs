pub mod exception_container;
use self::exception_container::ExceptionContainer;
use super::*;
use pyo3::{exceptions, types};
mod unit_tests;
mod validate;

#[pyclass]
#[derive(Debug)]
pub struct TemplateValidator {
    exceptions: Vec<ExceptionContainer>,
}

#[pymethods]
impl TemplateValidator {
    #[new]
    fn new(py: Python, error_classes: PyObject) -> PyResult<Self> {
        if let Ok(list) = error_classes.downcast::<types::PyList>(py) {
            Ok(Self {
                exceptions: list
                    .iter()
                    .map(|py_class| ExceptionContainer::new(py, py_class.into()))
                    .collect::<PyResult<Vec<_>>>()?,
            })
        } else {
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'List[ Class, Class... ]'",
                error_classes
            )));
        }
    }
}

