use crate::*;
use pyo3::exceptions::PyException;
use pyo3::types::{PyDict, PyTuple};
use std::collections::HashMap;

pub fn throw_error(obj: &PyObject, extra_hm: HashMap<String, String>) -> PyResult<()> {
    Python::with_gil(|py| {
        // Создаем объект класса ошибки с переданными параметрами
        let extra = PyDict::new(py);
        for (key, value) in extra_hm {
            extra.set_item(key, value)?;
        }
        let obj = obj
            .downcast::<PyAny>(py)?
            .call(PyTuple::empty(py), Some(extra))?;
        // Создаем объект класса & Возвращаем ошибку
        Err(PyErr::new::<PyException, _>(obj.into_py(py)))
    })
}
