use super::lazy_static;
use super::*;

/// Получаем extra из класса (MESSAGE_WITH_EXTRA_FROM_CLASS_PY)
fn extra_from_class(class_template: &types::PyType) -> PyResult<Vec<String>> {
    let attr_value = class_template
        .getattr(MESSAGE_WITH_EXTRA_FROM_CLASS_PY)?
        .to_string();
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

pub fn make_error(
    py: Python,
    custom_class_error: &PyObject,
    extra_with_value: &mut Option<HashMap<String, String>>,
) -> PyResult<()> {
    let extra = types::PyDict::new(py);
    let extra_from_class = extra_from_class(custom_class_error.downcast::<types::PyType>(py)?)?;
    if let Some(extra_with_value) = extra_with_value {
        extra_from_class.iter().for_each(|key| {
            extra_with_value.entry(key.into()).or_insert("___".into());
        });
        extra_with_value
            .iter()
            .try_for_each(|(key, value)| extra.set_item(key, value.as_str()))?;
    } else {
        extra_from_class.iter().for_each(|key| {
            extra.set_item(key, "___").unwrap();
        });
    }
    Err(PyErr::new::<PyException, _>(
        custom_class_error
            .downcast::<PyAny>(py)?
            .call(types::PyTuple::empty(py), Some(extra))?
            .into_py(py),
    )
    .into())
}
