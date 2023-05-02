use crate::*;
use pyo3::exceptions::{self, PyTypeError};
use std::collections::HashMap;
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

pub async fn regex_find(
    regex: &Regex,
    text: &str,
    extra: &Vec<String>,
) -> Option<HashMap<String, String>> {
    let mut result: HashMap<String, String> = HashMap::new();
    // получаем каждое совпадение
    for capture in regex.captures_iter(text) {
        // проверяем есть ли найденная группа из regex для **extra
        for name in extra {
            match capture.name(name) {
                // заполняем значение, если оно есть
                Some(value) => {
                    result.insert(name.to_string(), value.as_str().to_string());
                }
                // если нет, то в template вместо **extra выйдет "___"
                None => {
                    result.insert(name.to_string(), "___".to_string());
                }
            }
        }
    }
    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}
