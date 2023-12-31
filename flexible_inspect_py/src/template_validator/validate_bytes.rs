use super::*;
use crate::error::PyValidationErrorIterator;
use std::sync::Arc;

#[allow(clippy::type_complexity)]
#[pyclass(name = "TemplateValidatorBytes")]
pub struct PyTemplateValidatorBytes(Arc<TemplateValidator<Vec<Cartridge<RuleBytes>>, Arc<[u8]>>>);

#[pymethods]
impl PyTemplateValidatorBytes {
    #[new]
    pub fn new(cartridges: Vec<PyCartridgeBytes>) -> PyResult<Self> {
        Ok(Self(Arc::new(TemplateValidator::new(
            cartridges
                .into_iter()
                .map(|cartridge| cartridge.try_into())
                .collect::<PyResult<Vec<Cartridge<RuleBytes>>>>()?,
        ))))
    }

    pub fn validate(&self, data: &[u8]) -> Option<PyValidationErrorIterator> {
        if let Err(value) = self.0.validate(data.into()) {
            return Some(PyValidationErrorIterator::new(value.into_iter().collect()));
        }
        None
    }

    #[cfg(not(tarpaulin_include))]
    pub fn async_validate<'py>(&self, py: Python<'py>, data: &[u8]) -> PyResult<&'py PyAny> {
        let safety_self = Arc::clone(&self.0);
        let safety_data = Arc::from(data);
        pyo3_asyncio::async_std::future_into_py(py, async move {
            if let Err(value) = safety_self.async_validate(safety_data).await {
                return Ok(Some(PyValidationErrorIterator::new(
                    value.into_iter().collect(),
                )));
            }
            Ok(None)
        })
    }
}
