use crate::*;
use pyo3::exceptions::PyException;
use pyo3::types;
use std::collections::HashMap;

/// Создаем ошибку с переданными параметрами
pub fn init_error(obj: &PyObject, extra_hm: Option<HashMap<&str, &str>>) -> PyResult<PyObject> {
    Python::with_gil(|py| -> PyResult<PyObject> {
        dbg!(&extra_hm);
        // Создаем объект класса ошибки с переданными параметрами
        let extra = types::PyDict::new(py);
        if let Some(extra_hm) = extra_hm {
            for (key, value) in extra_hm {
                extra.set_item(key, value)?;
            }
        }
        let obj = obj.downcast::<types::PyType>(py)?;
        obj.setattr(EXTRA_FROM_CLASS_PY, extra)?;
        let obj = obj
            .downcast::<PyAny>()?
            .call(types::PyTuple::empty(py), Some(extra))?;
        // Создаем объект класса & Возвращаем ошибку
        Ok(PyErr::new::<PyException, _>(obj.to_object(py)).to_object(py))
    })
}
