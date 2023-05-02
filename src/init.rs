use crate::*;
use ::regex::Regex;
use lazy_static::lazy_static;
use pyo3::exceptions::{self, PyTypeError};
use std::collections::HashMap;
pub mod extra {
    use super::*;
    /// Получаем **extra переменные в виде коллекций
    pub fn extra_from_class<T: AsRef<str> + Send + Sync>(
        class_template: &types::PyType,
        attr: T,
    ) -> PyResult<Vec<String>> {
        let attr_value = class_template.getattr(attr.as_ref())?.to_string();
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{.+?\}").unwrap();
        }
        if RE.is_match(&attr_value) {
            Ok(RE
                .captures_iter(&attr_value)
                .map(|cap| {
                    cap.get(0).map_or(String::new(), |m| {
                        m.as_str().trim_matches('{').trim_matches('}').to_string()
                    })
                })
                .collect::<Vec<String>>())
        } else {
            return Err(exceptions::PyNameError::new_err(format!(
                "**extra is not defined (from `{}`)",
                MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
            )));
        }
    }
}
pub mod regex {
    use super::*;

    pub fn regex_from_class(class_template: &types::PyType) -> PyResult<Vec<String>> {
        let mut result = Vec::new();
        match class_template
            .getattr(RULES_FROM_CLASS_PY)?
            .downcast::<types::PyList>()
        {
            Ok(pylist) => {
                for item in pylist.iter() {
                    if let Ok(s) = item.extract::<String>() {
                        result.push(s);
                    }
                }
                Ok(result)
            }
            Err(_) => {
                return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                    "'{}' must be a List[ string, string... ]",
                    RULES_FROM_CLASS_PY
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
}
