use crate::*;
use lazy_static::lazy_static;
use pyo3::exceptions;
use regex::Regex;
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
            MESSAGE_EXTRA,
        )));
    }
}
// fn regex_from_class
