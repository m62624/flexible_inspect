mod export_py;
mod global_const;
mod init;
mod make_py_error;
mod move_not_copy;
use global_const::*;
use init::data;
use pyo3::prelude::*;
use pyo3::types;
use pyo3::ToPyObject;
use regex::Regex;
use std::str;
// Реалзиуем Clone, так как необходимо передавать `self` в `async fn`
#[derive(Debug, Clone)]
pub struct PythonSafeObject {
    // Храним переданный класс на основе которого создаем ошибку
    original_class: PyObject,
    // Храним **extra для заполнения информаций ошибки
    extra: Option<Vec<String>>,
    // Храним регулярные выражения для обработки текста
    rgxs: Vec<Regex>,
}
// Реалзиуем Clone, так как необходимо передавать `self` в `async fn`
#[derive(Debug, Clone)]
#[pyclass]
pub struct Validator {
    // Храним коллекцию, где структура обернута в `Box`,
    // так как необходимо будет запускать несколько `task` которые будут владеть этой структурой
    inner: Vec<PythonSafeObject>,
}
#[pymethods]
impl Validator {
    #[new]
    /// Конструктор для валидатора, принимает лист классов ошибок
    fn py_constructor(errors_list: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            Ok(Validator {
                inner: errors_list
                    .downcast::<types::PyList>(py)
                    .map_err(|e| e)?
                    .iter()
                    .map(|boxed_class| {
                        let obj = boxed_class.downcast::<types::PyType>()?;
                        let data = data::multi_data(&obj);
                        Ok(PythonSafeObject {
                            rgxs: data?.0.collect(),
                            extra: match data?.1 {
                                Some(value) => Some(value.collect()),
                                None => None,
                            },
                            original_class: obj.into(),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e: PyErr| e)?,
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
            slf.core_validate(Box::new(text)).await?;
            Ok(Python::with_gil(|py| py.None()))
        })
    }
}
impl Validator {
    pub async fn core_validate<'py>(&self, text_raw: Box<String>) -> PyResult<()> {
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
