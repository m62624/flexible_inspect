use crate::*;
use pyo3::exceptions::PyException;
use pyo3::types::{PyDict, PyTuple};
#[pyfunction]
pub fn throw_error(py: Python, error_class: &PyAny) -> PyResult<PyErr> {
    // get_attr_extra(error_class)?;
    // Создаем объект класса ошибки с переданными параметрами
    let kwargs = PyDict::new(py);
    let obj = error_class.call(PyTuple::empty(py), Some(kwargs))?;
    // Создаем объект класса & Возвращаем ошибку
    let shared_obj = obj.into_py(py);
    let pyerr = PyErr::new::<PyException, _>(shared_obj);
    Ok(pyerr)
}
