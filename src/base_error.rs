use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
#[derive(Debug)]
/// Структура которая будет являться шаблоном класса в `python` для создание собвственных ошибок
pub struct BaseError {
    message: String,
    extra: HashMap<String, String>,
    rules: HashMap<String, usize>,
}


#[pyclass(subclass)]
#[derive(Debug)]
/// Обертка для исключения Python
pub struct BaseErrorWrapper {
    inner: PyObject,
}

/// Реализация методов для `BaseErrorWrapper`
#[pymethods]
impl BaseErrorWrapper {
    /// Создаем экземпляр обертки с заданным исключением
    #[new]
    pub fn new(inner: PyObject) -> Self {
        BaseErrorWrapper { inner }
    }
}

/// Реализация преобразования из `BaseErrorWrapper` в `PyErr`
/// Для использования обернутого исключения в Python
impl From<BaseErrorWrapper> for PyErr {
    fn from(wrapper: BaseErrorWrapper) -> PyErr {
        PyErr::new::<PyException, _>(wrapper.inner)
    }
}
/// Функция для создания экземпляра обертки `BaseErrorWrapper`
#[pyfunction]
pub fn throw_base_error(
    _py: Python<'_>,
    message: String,
    extra: HashMap<String, String>,
    rules: HashMap<String, usize>,
) -> PyResult<BaseErrorWrapper> {
    Python::with_gil(|py| {
        let base_error = BaseError {
            message,
            extra,
            rules,
        };
        let pyerr: PyErr = PyErr::new::<BaseError, _>(base_error);
        Ok(BaseErrorWrapper::new(pyerr.to_object(py)))
    })
}
