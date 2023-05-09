use crate::*;
use lazy_static::lazy_static;
use pyo3::exceptions::PyException;
use pyo3::types::{PyDict, PyTuple};
use std::collections::HashMap;

pub fn throw_error(obj: &PyObject, extra_hm: Option<HashMap<String, String>>) -> PyResult<()> {
    Python::with_gil(|py| {
        // Создаем объект класса ошибки с переданными параметрами
        let extra = PyDict::new(py);
        if let Some(extra_hm) = extra_hm {
            for (key, value) in extra_hm {
                extra.set_item(key, value)?;
            }
        }
        let obj = obj
            .downcast::<PyAny>(py)?
            .call(PyTuple::empty(py), Some(extra))?;
        // Создаем объект класса & Возвращаем ошибку
        Err(PyErr::new::<PyException, _>(obj.into_py(py)))
    })
}
pub fn extra_from_class<'a, T: AsRef<str>>(
    class_template: &types::PyType,
    attr: T,
) -> PyResult<Option<Vec<String>>> {
    let attr_value = class_template.getattr(attr.as_ref())?.to_string();
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"\{.+?\}").unwrap();
    }
    if RE.is_match(&attr_value) {
        Ok(Some(
            RE.captures_iter(&attr_value)
                .map(|cap| {
                    cap.get(0).map_or(String::new(), |m| {
                        m.as_str().trim_matches('{').trim_matches('}').to_string()
                    })
                })
                .collect::<Vec<_>>(),
        ))
    } else {
        Ok(None)
    }
}

// pub async fn regex_find(
//     regex: &regex::Regex,
//     text: &str,
//     extra: &Vec<String>,
// ) -> Option<HashMap<String, String>> {
//     let mut result: HashMap<String, String> = HashMap::new();
//     // получаем каждое совпадение
//     for capture in regex.captures_iter(text) {
//         // проверяем есть ли найденная группа из regex для **extra
//         for name in extra {
//             match capture.name(name) {
//                 // заполняем значение, если оно есть
//                 Some(value) => {
//                     result.insert(name.to_string(), value.as_str().to_string());
//                 }
//                 // если нет, то в template вместо **extra выйдет "___"
//                 None => {
//                     result.insert(name.to_string(), "___".to_string());
//                 }
//             }
//         }
//     }
//     if !result.is_empty() {
//         Some(result)
//     } else {
//         None
//     }
// }
