use crate::error::PyValidationErrorIterator;

use super::*;
use std::sync::Arc;

#[pyclass(name = "TemplateValidatorBytes")]
pub struct PyTemplateValidatorBytes(TemplateValidator<Vec<Cartridge<RuleBytes>>, Arc<[u8]>>);

#[pymethods]
impl PyTemplateValidatorBytes {
    #[new]
    pub fn new(cartridges: Vec<PyCartridgeBytes>) -> Self {
        Self(TemplateValidator::new(
            cartridges
                .into_iter()
                .map(|cartridge| cartridge.into())
                .collect(),
        ))
    }

    pub fn validate(&self, data: &[u8]) -> PyValidationErrorIterator {
        if let Err(value) = self.0.validate(data.into()) {
            return PyValidationErrorIterator::new(value.into_iter().collect());
        }
        PyValidationErrorIterator::new(vec![])
    }

    pub fn async_validate<'py>(&self, py: Python, data: &[u8]) -> PyResult<&'py PyAny> {
        pyo3_asyncio::async_std::future_into_py(py, async move {
            if let Err(value) = self.0.async_validate(data.into()).await {
                return Ok(PyValidationErrorIterator::new(value.into_iter().collect()));
            }
            Ok(PyValidationErrorIterator::new(vec![]))
        })
    }
}
