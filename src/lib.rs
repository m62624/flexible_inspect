mod export_py;
mod extra_init;
mod pyst_errors;
mod regex_init;
use async_std;
use async_std::channel;
use pyo3::prelude::*;
use pyo3::types;
use regex::Regex;
use std::str;
use std::sync::Arc;
//==============================================
// имя атрибута, где хранится само сообщение и **extra переменные
const MESSAGE_EXTRA: &'static str = "template";
// имя атрибута, где хранятся регулярные выражения
const REGEGX_RULES: &'static str = "rules";
//==============================================
// Реалзиуем Clone, так как необходимо передавать `self` в `async fn`
#[derive(Debug, Clone)]
pub struct PythonSafeObject {
    // Храним переданный класс на основе которого создаем ошибку
    original_class: PyObject,
    // Храним **extra для заполнения информаций ошибки
    extra: Vec<String>,
    // Храним регулярные выражения для обработки текста
    regex_collection: Vec<Regex>,
}
// Реалзиуем Clone, так как необходимо передавать `self` в `async fn`
#[derive(Debug, Clone)]
#[pyclass]
pub struct Validator {
    // Храним коллекцию, где структура обернута в `Arc`,
    // так как необходимо будет запускать несколько `task` которые будут владеть этой структурой
    inner: Vec<Arc<PythonSafeObject>>,
}
// Модуль для инициализаций `Validator`
mod init_validator {
    use super::*;
    #[pymethods]
    impl Validator {
        // Конструктор сразу получает `extra`, `regex`, `class` и проверяет их корректность
        #[new]
        #[pyo3(signature=(errors_list))]
        pub fn py_constructor(errors_list: PyObject) -> PyResult<Self> {
            Python::with_gil(|py| {
                // Создаем коллекцию классов для будущего построения ошибок на их основе
                let mut class_templates: Vec<Arc<PythonSafeObject>> = Vec::new();
                // Получаем каждый класс как `PyAny` и итеративно обрабатываем его
                for boxed_class in errors_list.downcast::<types::PyList>(py)?.iter() {
                    // Переводим из `PyAny` в `PyType`, чтобы получить доступ к атрибутам
                    let unboxed_class = boxed_class.downcast::<types::PyType>()?;
                    // Сохраняем структуру как `Arc` для `async fn`
                    // Сразу получаем все нужный атрибуты от каждого класса по отдельности
                    class_templates.push(Arc::new(PythonSafeObject {
                        // Обратно возращаем класс в `PyObject`, чтобы можно было в будущем использовать его снова
                        original_class: unboxed_class.to_object(py),
                        // Получаем все `**extra`
                        extra: extra_init::extra_from_class(unboxed_class, MESSAGE_EXTRA)?,
                        // Получаем все регулярные выражения
                        regex_collection: regex_init::regex_from_class(unboxed_class)?,
                    }));
                }
                Ok(Validator {
                    inner: class_templates,
                })
            })
        }
        // Служит для запуска async метода
        #[pyo3(name = "validate")]
        #[pyo3(signature =(text_bytes))]
        fn py_validate<'py>(
            &self,
            py: Python<'py>,
            text_bytes: &types::PyBytes,
        ) -> PyResult<&'py PyAny> {
            // Необходимо использовать `to_owned()`, так как асинхронный метод
            // требует владения над `self`
            let (slf, text) = (
                self.to_owned(),
                Arc::new(str::from_utf8(text_bytes.as_bytes()).unwrap().to_string()),
            );
            // Переводим `Future` в `python` аналог
            pyo3_asyncio::async_std::future_into_py(py, async move {
                slf.core_validate(text).await?;
                Ok(Python::with_gil(|py| py.None()))
            })
        }
        /// минимальный набор методов для отладки (__repr__, __str__)
        pub fn __repr__(&self) -> PyResult<String> {
            Ok(format!("Validator({:#?})", self.inner))
        }
        pub fn __str__(&self) -> String {
            format!("{:#?}", self.inner)
        }
    }
}

mod validate_async {
    use super::*;

    impl Validator {
        // Асинхронныый метод для проверки текста, основываясь на регулярных выражениях
        pub async fn core_validate<'py>(&self, text_raw: Arc<String>) -> PyResult<()> {
            let channel_for_error = channel::unbounded();
            let channel_clone = Arc::new(channel_for_error.0);
            // здесь будем хранить каждый класс ошибки для асинхронного выполнения
            let mut tasks = Vec::new();
            // Проходимся по каждой ошибке для проверки
            for item in self.inner.iter() {
                let sender_clone = Arc::clone(&channel_clone);
                // делаем ARC от item (одна ошибка) чтобы отправить в task
                let item_clone = Arc::clone(&item);
                // делаем ARC от text_raw (одна ошибка) чтобы отправить в task
                let text_clone = Arc::clone(&text_raw);
                // создаем task, чтобы асинхронно выполнить каждую ошибку отдельно
                let task = async_std::task::spawn(async move {
                    // получаем каждый regex для проверки
                    for pattern in &item_clone.regex_collection {
                        if let Some(extra) =
                            regex_init::regex_find(pattern, &text_clone, &item_clone.extra).await
                        {
                            let x = pyst_errors::throw_error(&item_clone.original_class, extra);
                            sender_clone.send(x).await.unwrap();
                        }
                    }
                });
                tasks.push(task);
            }
            // Запускаем каждый task асинхронно
            for lazy_task in tasks {
                lazy_task.await;
                channel_for_error.1.recv().await.unwrap()?;
            }
            Ok(())
        }
    }
}
