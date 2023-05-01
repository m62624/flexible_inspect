mod export_py;
mod extra_init;
mod regex_init;
use async_std;
use pyo3::prelude::*;
use pyo3::types;
use regex::Regex;
use std::str;
use std::sync::Arc;
//==============================================
// хранит само сообщение и **extra переменные
const MESSAGE_EXTRA: &'static str = "template";
// хранит регулярные выражения, является правилами обнаружения ошибок
const REGEGX_RULES: &'static str = "rules";
//==============================================
#[derive(Debug, Clone)]
pub struct PythonSafeObject {
    original_class: PyObject,
    extra: Vec<String>,
    regex_collection: Vec<Regex>,
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Validator {
    inner: Vec<Arc<PythonSafeObject>>,
}
mod init_validator {
    use super::*;
    #[pymethods]

    impl Validator {
        #[new]
        #[pyo3(signature =(errors_list))]
        pub fn py_constructor(errors_list: PyObject) -> PyResult<Self> {
            Python::with_gil(|py| {
                let mut class_templates: Vec<Arc<PythonSafeObject>> = Vec::new();
                for boxed_class in errors_list.downcast::<types::PyList>(py)?.iter() {
                    let unboxed_class = boxed_class.downcast::<types::PyType>()?;
                    class_templates.push(Arc::new(PythonSafeObject {
                        original_class: unboxed_class.to_object(py),
                        extra: extra_init::extra_from_class(unboxed_class, MESSAGE_EXTRA)?,
                        regex_collection: regex_init::regex_from_class(unboxed_class)?,
                    }));
                }
                let temp = Validator {
                    inner: class_templates,
                };
                dbg!(&temp);
                Ok(temp)
            })
        }
        #[pyo3(name = "validate")]
        #[pyo3(signature =(text_bytes))]
        fn py_validate<'py>(
            &self,
            py: Python<'py>,
            text_bytes: &types::PyBytes,
        ) -> PyResult<&'py PyAny> {
            let (slf, text) = (
                self.to_owned(),
                Arc::new(str::from_utf8(text_bytes.as_bytes()).unwrap().to_string()),
            );
            pyo3_asyncio::async_std::future_into_py(py, async move {
                slf.core_validate(text).await;
                Ok(Python::with_gil(|py| py.None()))
            })
        }
        /// минимальный набор методов для отладки
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
        pub async fn core_validate<'py>(&self, text_raw: Arc<String>) {
            // здесь будем хранить каждый класс ошибки для асинхронного выполнения
            let mut tasks = Vec::new();
            // Проходимся по каждой ошибке для проверки
            for item in self.inner.iter() {
                // делаем ARC от item (одна ошибка) чтобы отправить в task
                let item_clone = Arc::clone(&item);
                // делаем ARC от text_raw (одна ошибка) чтобы отправить в task
                let text_clone = Arc::clone(&text_raw);
                // создаем task, чтобы асинхронно выполнить каждую ошибку отдельно
                let task = async_std::task::spawn(async move {
                    // получаем каждый regex для проверки
                    for pattern in &item_clone.regex_collection {
                        regex_init::regex_find(pattern, &text_clone);
                    }
                });
                tasks.push(task);
            }
            // Запускаем каждый task асинхронно
            for lazy_task in tasks {
                lazy_task.await;
            }
        }
    }
}
