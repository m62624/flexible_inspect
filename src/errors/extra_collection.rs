use std::collections::HashMap;

use lazy_static::lazy_static;
use pyo3::prelude::*;
use pyo3::types;
/// Получаем extra из класса (MESSAGE_WITH_EXTRA_FROM_CLASS_PY)
pub fn extra_from_class<'a>(
    class_template: &'a types::PyType,
    attr: &str,
) -> PyResult<Vec<&'a str>> {
    let attr_value = class_template.getattr(attr)?.extract::<&str>()?;
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"\{.+?\}").unwrap();
    }
    if RE.is_match(&attr_value) {
        Ok(RE
            .captures_iter(&attr_value)
            .map(|cap| {
                cap.get(0)
                    .map_or("", |m| m.as_str().trim_matches('{').trim_matches('}'))
            })
            .collect())
    } else {
        Ok(Vec::new())
    }
}

pub fn extra_from_default_capture<'a>(
    extra_name: Vec<&'a str>,
    captures: Vec<regex::Captures<'a>>,
) -> Option<HashMap<&'a str, &'a str>> {
    let mut extra_values: HashMap<&str, &str> = HashMap::new();
    captures
        .iter()
        .map(|capture| {
            extra_name
                .iter()
                .map(|name| match capture.name(name) {
                    Some(value) => {
                        extra_values.insert(name, value.as_str());
                    }
                    None => {
                        extra_values.insert(name, "___");
                    }
                })
                .for_each(drop);
        })
        .for_each(drop);
    if !extra_values.is_empty() {
        return Some(extra_values);
    }
    return None;
}
pub fn extra_from_default_fancy<'a>(
    extra_name: &'a Vec<&'a str>,
    captures: Vec<fancy_regex::Captures<'a>>,
) -> Option<HashMap<&'a str, &'a str>> {
    let mut extra_values: HashMap<&str, &str> = HashMap::new();
    captures
        .iter()
        .map(|capture| {
            extra_name
                .iter()
                .map(|name| match capture.name(name) {
                    Some(value) => {
                        extra_values.insert(name, value.as_str());
                    }
                    None => {
                        extra_values.insert(name, "___");
                    }
                })
                .for_each(drop);
        })
        .for_each(drop);
    if !extra_values.is_empty() {
        return Some(extra_values);
    }
    return None;
}
