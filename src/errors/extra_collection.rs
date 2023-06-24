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
