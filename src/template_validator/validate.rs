use super::*;

impl TemplateSafeSelf {
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        let mut data = Vec::new();
        if let Ok(list) = cartridges.downcast::<types::PyList>(py) {
            list.iter().try_for_each(|class_obj| {
                if let Ok(class_py) = class_obj.downcast::<types::PyType>() {
                    data.push(CartridgeWrapper::new(py, class_py.to_object(py))?);
                    Ok(())
                } else {
                    Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "'{}' must be a 'Class'",
                        class_obj
                    )))
                }
            })?;
            Ok(Self { cartridges: data })
        } else {
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'List[ Class, Class... ]'",
                cartridges
            )));
        }
    }
}

#[pymethods]
impl TemplateValidator {
    pub fn validate(&self, py: Python, text_bytes: &types::PyBytes) -> PyResult<Option<PyObject>> {
        let text = Self::bytes_to_string_utf8(text_bytes.as_bytes())?;
        let mut errors = Vec::new();
        for cartridge in &self.0.cartridges {
            if let NextStep::Error(value) = cartridge.sync_run(&text) {
                if let Err(err) =
                    custom_error::make_error(cartridge.get_cartridge().get_py_class(), value)
                {
                    errors.push(err);
                }
            }
        }
        if !errors.is_empty() {
            return Ok(Some(errors.into_py(py)));
        }
        Ok(None)
    }
    pub fn async_validate<'py>(
        &self,
        py: Python<'py>,
        text_bytes: &types::PyBytes,
    ) -> PyResult<&'py PyAny> {
        let text = Self::bytes_to_string_utf8(text_bytes.as_bytes())?;
        let async_self = Arc::clone(&self.0);
        pyo3_asyncio::async_std::future_into_py(py, async move {
            for cartridge in &async_self.cartridges {
                if let NextStep::Error(value) = cartridge.async_run(&text).await {
                    custom_error::make_error(cartridge.get_cartridge().get_py_class(), value)?;
                }
            }
            Ok(Python::with_gil(|py| py.None()))
        })
    }
}

impl TemplateValidator {
    fn bytes_to_string_utf8(bytes: &[u8]) -> PyResult<String> {
        match String::from_utf8(bytes.into()) {
            Ok(result) => Ok(result),
            Err(_) => Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                "{:#?} is not a valid UTF-8 string",
                bytes
            ))),
        }
    }
}
