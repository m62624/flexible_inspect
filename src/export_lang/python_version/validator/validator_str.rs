use super::*;

#[pyclass(name = "TemplateValidator")]
pub struct PyTemplateValidator(Arc<PyTemplateValidatorAsync>);
pub struct PyTemplateValidatorAsync(TemplateValidator<Vec<Cartridge<Rule>>, Arc<str>>);

impl PyTemplateValidatorBaseRust for PyTemplateValidator {
    type RustCartridgeType = Cartridge<Rule>;
}

#[pymethods]
impl PyTemplateValidator {
    #[new]
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        Ok(Self(Arc::new(PyTemplateValidatorAsync(
            TemplateValidator::new(PyTemplateValidator::_to_rust_for_new::<PyCartridge>(
                py,
                cartridges,
                "Cartridge",
            )?),
        ))))
    }

    fn validate(&self, data: String) -> Option<Vec<PyErr>> {
        self.0._validate(data)
    }

    fn async_validate<'py>(&self, py: Python<'py>, data: String) -> PyResult<&'py PyAny> {
        let slf = Arc::clone(&self.0);
        pyo3_asyncio::async_std::future_into_py(
            py,
            async move { Ok(slf._async_validate(data).await) },
        )
    }
}

#[async_trait]
impl PyTemplateValidatorBase<String> for PyTemplateValidatorAsync {
    fn _validate(&self, data: String) -> Option<Vec<PyErr>> {
        trace!("{:#?}", &self.0.cartridges.iter());
        let mut error = Vec::new();
        for cartridge in self.0.cartridges.iter() {
            if let NextStep::Error(extra_with_value) = cartridge.run(data.as_str()) {
                error.push(
                    PyPystvalError::new(
                        <Cartridge<Rule> as CartridgeBase<Rule, &str>>::get_id(cartridge),
                        filling_message(
                            <Cartridge<Rule> as CartridgeBase<Rule, &str>>::get_message(cartridge),
                            extra_with_value,
                        ),
                    )
                    .into(),
                );
            }
        }
        if error.is_empty() {
            None
        } else {
            Some(error)
        }
    }

    async fn _async_validate(&self, data: String) -> Option<Vec<PyErr>> {
        self._validate(data)
    }
}
