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
 use crate::*;
 use pyo3::exceptions::PyException;
 use pyo3::types::PyType;
pub fn get_extra<T: AsRef<str> + Send + Sync>(
    obj: &PyType,
    attr: T,
    regex: T,
) -> PyResult<Vec<String>> {
    let (pattern, attr_value) = (
        Regex::new(regex.as_ref()).unwrap(),
        obj.getattr(attr.as_ref())?.to_string(),
    );
    if pattern.is_match(&attr_value) {
        Ok(pattern
            .captures_iter(&attr_value)
            .map(|cap| {
                cap.get(0).map_or(String::new(), |m| {
                    m.as_str().trim_matches('{').trim_matches('}').to_string()
                })
            })
            .collect::<Vec<String>>())
    } else {
        return Err(exceptions::PyNameError::new_err(format!(
            "**extra is not defined (from `template`)",
        )));
    }
 }
