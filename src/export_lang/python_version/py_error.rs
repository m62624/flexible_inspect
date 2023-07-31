use crate::core::base_error::PystvalError;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::pyclass;
use std::fmt;

#[pyclass(name = "PystvalException")]
#[derive(Debug)]
pub struct PyPystvalError(PystvalError);

impl PyPystvalError {
    pub fn new(id: i64, msg: String) -> Self {
        Self(PystvalError::new(id, msg))
    }
}

#[pymethods]
impl PyPystvalError {
    pub fn get_code(&self) -> i64 {
        self.0.id
    }
}

impl fmt::Display for PyPystvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::convert::From<PyPystvalError> for PyErr {
    fn from(value: PyPystvalError) -> Self {
        PyException::new_err(value.to_string())
    }
}
