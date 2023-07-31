use super::*;

#[pyclass(name = "TemplateValidatorBytes")]
pub struct PyTemplateValidatorBytes(Arc<PyTemplateValidatorBytesAsync>);
pub struct PyTemplateValidatorBytesAsync(TemplateValidator<Vec<Cartridge<RuleBytes>>, Arc<[u8]>>);

impl PyTemplateValidatorBaseRust for PyTemplateValidatorBytes {
    type RustCartridgeType = Cartridge<RuleBytes>;
}

#[pymethods]
impl PyTemplateValidatorBytes {
    #[new]
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        Ok(Self(Arc::new(PyTemplateValidatorBytesAsync(
            TemplateValidator::new(PyTemplateValidatorBytes::_to_rust_for_new::<
                PyCartridgeBytes,
            >(py, cartridges, "CartridgeBytes")?),
        ))))
    }

    fn validate(&self, data: Vec<u8>) -> Option<Vec<PyPystvalError>> {
        self.0._validate(data)
    }

    fn async_validate<'py>(&self, py: Python<'py>, data: Vec<u8>) -> PyResult<&'py PyAny> {
        let slf = Arc::clone(&self.0);
        pyo3_asyncio::async_std::future_into_py(
            py,
            async move { Ok(slf._async_validate(data).await) },
        )
    }
}

#[async_trait]
impl PyTemplateValidatorBase<Vec<u8>> for PyTemplateValidatorBytesAsync {
    fn _validate(&self, data: Vec<u8>) -> Option<Vec<PyPystvalError>> {
        trace!("{:#?}", &self.0.cartridges.iter());
        let mut error = Vec::new();
        for cartridge in self.0.cartridges.iter() {
            if let NextStep::Error(extra_with_value) = cartridge.run(data.as_slice()) {
                error.push(PyPystvalError::new(
                    <Cartridge<RuleBytes> as CartridgeBase<RuleBytes, &[u8]>>::get_id(cartridge),
                    filling_message(
                        <Cartridge<RuleBytes> as CartridgeBase<RuleBytes, &[u8]>>::get_message(
                            cartridge,
                        ),
                        extra_with_value,
                    ),
                ));
            }
        }
        if error.is_empty() {
            None
        } else {
            Some(error)
        }
    }

    async fn _async_validate(&self, data: Vec<u8>) -> Option<Vec<PyPystvalError>> {
        self._validate(data)
    }
}
