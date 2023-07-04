use super::*;

#[pymethods]
impl TemplateValidator {
    pub fn validate(&self, py: Python, text_bytes: &types::PyBytes) -> PyResult<Option<PyObject>> {
        let text = Self::bytes_to_string_utf8(text_bytes.as_bytes())?;
        let mut errors = Vec::new();
        for cartridge in &self.0.cartridges {
            if let NextStep::Error(mut value) = cartridge.sync_run(&text) {
                if let Err(err) = custom_error::make_error(
                    py,
                    cartridge.get_cartridge().get_py_class(),
                    &mut value,
                ) {
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
        let text = Arc::new(Self::bytes_to_string_utf8(text_bytes.as_bytes())?);
        let async_self = Arc::clone(&self.0);
        // println!("Запущена функция для future into py");
        pyo3_asyncio::async_std::future_into_py(py, async move {
            async_std::task::spawn_blocking(|| async move {
                // println!("Запустился отедльный таск в потоке");
                for cartridge in &async_self.cartridges {
                    if let NextStep::Error(mut value) = cartridge.async_run(Arc::clone(&text)).await
                    {
                        // println!("Зарегистрировал ошибку");
                        Python::with_gil(|py| -> PyResult<()> {
                            custom_error::make_error(
                                py,
                                cartridge.get_cartridge().get_py_class(),
                                &mut value,
                            )
                        })?
                    }
                }
                Ok::<(), PyErr>(())
            })
            .await
            .await?;
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
