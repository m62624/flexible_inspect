use crate::*;

use pyo3::exceptions::{self, PyTypeError};
pub fn regex_from_class(class_template: &types::PyType) -> PyResult<Vec<Regex>> {
    let mut result = Vec::new();
    match class_template
        .getattr(REGEGX_RULES)?
        .downcast::<types::PyList>()
    {
        Ok(pylist) => {
            for item in pylist.iter() {
                if let Ok(s) = item.extract::<String>() {
                    result.push(get_regex(s)?);
                }
            }
            Ok(result)
        }
        Err(_) => {
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a List[ string, string... ]",
                REGEGX_RULES
            )))
        }
    }
}

pub fn get_regex<T: AsRef<str> + Send + Sync>(regex_template: T) -> PyResult<Regex> {
    match Regex::new(regex_template.as_ref()) {
        Ok(value) => return Ok(value),
        Err(_) => Err(PyErr::new::<PyTypeError, _>("Invalid regular expression")),
    }
}
