use super::exception_container::ExceptionContainer;
use super::*;

#[pyclass]
pub struct TemplateValidator {
    exceptions: Vec<ExceptionContainer>,
}

impl TemplateValidator {
    pub fn get_exceptions(&self) -> &Vec<ExceptionContainer> {
        &self.exceptions
    }
    pub fn bytes_to_string_utf8(bytes: &[u8]) -> PyResult<String> {
        match String::from_utf8(bytes.into()) {
            Ok(result) => Ok(result),
            Err(_) => Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                "{:#?} is not a valid UTF-8 string",
                bytes
            ))),
        }
    }
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
