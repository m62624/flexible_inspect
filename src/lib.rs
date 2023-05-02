mod export_py;
mod global_const;
mod init;
mod make_py_error;
mod move_not_copy;
use global_const::*;
use pyo3::prelude::*;
use pyo3::types;
use pyo3::ToPyObject;
use std::str;
use std::sync::Arc;
// Реалзиуем Clone, так как необходимо передавать `self` в `async fn`
#[derive(Debug, Clone)]
pub struct PythonSafeObject {
    // Храним переданный класс на основе которого создаем ошибку
    original_class: PyObject,
    // Храним **extra для заполнения информаций ошибки
    extra: Vec<String>,
    // Храним регулярные выражения для обработки текста
    rgxs: Vec<String>,
}
// Реалзиуем Clone, так как необходимо передавать `self` в `async fn`
#[derive(Debug, Clone)]
#[pyclass]
pub struct Validator {
    // Храним коллекцию, где структура обернута в `Arc`,
    // так как необходимо будет запускать несколько `task` которые будут владеть этой структурой
    inner: Vec<Arc<PythonSafeObject>>,
}
#[pymethods]
impl Validator {
    #[new]
    /// Конструктор для валидатора, принимает лист классов ошибок
    fn py_constructor(errors_list: PyObject) -> PyResult<Self> {
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
                    extra: init::extra::extra_from_class(
                        unboxed_class,
                        MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
                    )?,
                    // Получаем все регулярные выражения
                    rgxs: init::regex::regex_from_class(unboxed_class)?,
                }));
            }
            Ok(Validator {
                inner: class_templates,
            })
        })
    }
    // Служит для запуска async метода
    #[pyo3(name = "validate")]
    #[pyo3(signature = (text_bytes))]
    fn py_validate<'py>(
        &self,
        py: Python<'py>,
        text_bytes: &types::PyBytes,
    ) -> PyResult<&'py PyAny> {
        // let slf = self.self_context(py)?;
        let slf = self.clone();
        let text = init::bytes_utf8::convert(text_bytes)?;
        pyo3_asyncio::async_std::future_into_py(py, async move {
            slf.core_validate(Arc::new(text)).await?;
            Ok(Python::with_gil(|py| py.None()))
        })
    }
}
impl Validator {
    pub async fn core_validate<'py>(&self, text_raw: Arc<String>) -> PyResult<()> {
        let mut result: Result<(), PyErr> = Ok(());
        for obj in self.inner.iter() {
            for rgx in &obj.rgxs {
                if let Some(extra) =
                    init::regex::regex_find(&init::regex::get_regex(rgx)?, &text_raw, &obj.extra)
                        .await
                {
                    result = make_py_error::throw_error(&obj.original_class, extra);
                }
            }
        }
        result
    }
    pub fn self_context(&self, py: Python) -> PyResult<Validator> {
        self.to_object(py).extract::<Validator>(py)
    }
}
