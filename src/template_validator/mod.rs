use super::*;
mod exception_contrainer;
use exception_contrainer::ExceptionContainer;
use pyo3::{exceptions, types};
pub mod captures;
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
            let mut exceptions = Vec::new();
            list.iter()
                .map(|py_class| {
                    exceptions.push(ExceptionContainer::new(py, py_class.into())?);
                    Ok(())
                })
                .collect::<PyResult<Vec<_>>>()?;
            Ok(Self { exceptions })
        } else {
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'List[ Class, Class... ]'",
                error_classes
            )));
        }
    }
}
