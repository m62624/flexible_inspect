use crate::*;
use lazy_static::lazy_static;
use pyo3::exceptions::PyException;
use pyo3::types::PyDict;
use std::collections::HashMap;

pub fn create_error(obj: &PyObject, extra_hm: Option<HashMap<String, String>>) -> PyResult<()> {
    Python::with_gil(|py| {
        dbg!(&extra_hm);
        // Создаем объект класса ошибки с переданными параметрами
        let extra = PyDict::new(py);
        if let Some(extra_hm) = extra_hm {
            for (key, value) in extra_hm {
                extra.set_item(key, value)?;
            }
        }
        let obj = obj.downcast::<types::PyType>(py)?;
        obj.setattr(EXTRA_FROM_CLASS_PY, extra)?;
        // Создаем объект класса & Возвращаем ошибку
        Err(PyErr::new::<PyException, _>(obj.to_object(py)))
    })
}
pub fn extra_from_class<'a, T: AsRef<str>>(
    class_template: &types::PyType,
    attr: T,
) -> PyResult<Vec<String>> {
    let attr_value = class_template.getattr(attr.as_ref())?.to_string();
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"\{.+?\}").unwrap();
    }
    if RE.is_match(&attr_value) {
        Ok(RE
            .captures_iter(&attr_value)
            .map(|cap| {
                cap.get(0).map_or(String::new(), |m| {
                    m.as_str().trim_matches('{').trim_matches('}').to_string()
                })
            })
            .collect::<Vec<_>>())
    } else {
        Ok(Vec::new())
    }
}

pub fn error_or_ok(
    obj: &PyObject,
    extra_values: HashMap<String, String>,
    rule_status: &RuleStatus,
    flag: bool,
) -> PyResult<()> {
    let error = || {
        if extra_values.is_empty() {
            make_errors::create_error(&obj, None)
        } else {
            make_errors::create_error(&obj, Some(extra_values))
        }
    };
    match (&rule_status.status, flag) {
        (It::MustBeFoundHere, true) => Ok(()),
        (It::NotToBeFoundHere, false) => Ok(()),
        (It::MustBeFoundHere, false) => error(),
        (It::NotToBeFoundHere, true) => error(),
    }
}
