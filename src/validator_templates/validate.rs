use super::*;

#[pymethods]
impl TemplateValidator {
    #[cfg(not(tarpaulin_include))]
    pub fn async_validate<'py>(
        &self,
        py: Python<'py>,
        text_bytes: &types::PyBytes,
    ) -> PyResult<&'py PyAny> {
        let text = bytes_to_string_utf8(text_bytes.as_bytes())?;
        let async_self = Arc::clone(&self.0);
        pyo3_asyncio::async_std::future_into_py(py, async move {
            // unsafe_self.core_validate(text)?;
            for cartridge in &async_self.cartridges {
                cartridge.async_run(&text).await?;
            }
            Ok(Python::with_gil(|py| py.None()))
        })
    }

    #[cfg(not(tarpaulin_include))]
    pub fn validate(&self, text_bytes: &types::PyBytes) -> PyResult<()> {
        let text = bytes_to_string_utf8(text_bytes.as_bytes())?;
        for cartridge in &self.0.cartridges {
            cartridge.sync_run(&text)?;
        }
        Ok(())
    }
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
