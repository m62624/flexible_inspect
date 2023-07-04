use super::*;

/// Реализация методов валидаций для API `Python`
#[pymethods]
impl TemplateValidator {
    /// Synchronous text validation, unlike `async` versions,
    /// goes through all classes, and returns a sheet, with errors based on the classes
    ///
    /// Main plus, it returns the entire list of failed classes at once
    pub fn validate(&self, py: Python, text_bytes: &types::PyBytes) -> PyResult<Option<PyObject>> {
        // Проверяем байты на `UTF-8`
        let text = Self::bytes_to_string_utf8(text_bytes.as_bytes())?;
        // Коллекция для хранения ошибок
        let mut errors = Vec::new();
        // Проходимся по всем классам
        for cartridge in &self.0.cartridges {
            // Если валидация не прошла, то создаем ошибку
            if let NextStep::Error(mut value) = cartridge.sync_run(&text) {
                if let Err(err) = custom_error::make_error(
                    py,
                    cartridge.get_cartridge().get_py_class(),
                    &mut value,
                ) {
                    // Если ошибка, то добавляем ее в коллекцию
                    errors.push(err);
                }
            }
        }
        // Если есть ошибки, то возвращаем их
        if !errors.is_empty() {
            return Ok(Some(errors.into_py(py)));
        }
        Ok(None)
    }

    /// Asynchronous text checking, unlike `sync` versions, goes class by class, but immediately stops working at the first mismatch and returns a single error class
    ///
    /// Main plus, multiple rules work competitively in a single validation
    pub fn async_validate<'py>(
        &self,
        py: Python<'py>,
        text_bytes: &types::PyBytes,
    ) -> PyResult<&'py PyAny> {
        // Проверяем байты на `UTF-8` и создаем `Arc<String>` для `async task`
        let text = Arc::new(Self::bytes_to_string_utf8(text_bytes.as_bytes())?);
        // Получаем ссылку на `self`, для `async task`
        let async_self = Arc::clone(&self.0);
        // println!("Запущена функция для future into py");
        // Возвращаем `future into py`, необходимо для того чтобы, создать `awaitable object`
        // для `python`
        pyo3_asyncio::async_std::future_into_py(py, async move {
            // Итак, почему это выглядит именно так а не иначе ?

            /*
            Поскольку `Python` не имеет концепции владения и работает исключительно с объектами в штучной упаковке, на любой объект Python можно ссылаться любое количество раз, и разрешено изменение любой ссылки.

            Ситуацию облегчает глобальная блокировка интерпретатора (GIL), которая гарантирует, что только один поток может одновременно использовать интерпретатор Python и его API. Это означает, что вам не нужно беспокоиться о том, что один поток изменит объект, когда другой поток использует его.
             */

            // Поэтому мы получаем маркер (token) интерпретатора `GIL` и выполняем в нем `async task`
            // При получения токена, блокируется поток, поэтому мы используем `spawn_blocking`
            // для того чтобы не блокировать поток, в котором выполняется `Python`
            async_std::task::spawn_blocking(|| async move {
                // println!("Запустился отедльный таск в потоке");
                for cartridge in &async_self.cartridges {
                    if let NextStep::Error(mut value) = cartridge.async_run(Arc::clone(&text)).await
                    {
                        // println!("Зарегистрировал ошибку");
                        // Полуаем `GIL` и создаем ошибку
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
    /// Проверяем байты на `UTF-8`
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
