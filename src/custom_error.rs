use super::*;

fn get_extra_from_class<'a>(
    py: Python<'a>,
    class_template: &'a PyObject,
) -> PyResult<Vec<&'a str>> {
    let attr_value = class_template
        .downcast::<types::PyType>(py)
        .unwrap()
        .getattr(MESSAGE_WITH_EXTRA_FROM_CLASS_PY)?
        .extract::<&str>()?;
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"\{.+?\}").unwrap();
    }
    if RE.is_match(attr_value) {
        Ok(RE
            .captures_iter(attr_value)
            .map(|cap| {
                cap.get(0)
                    .map_or("", |m| m.as_str().trim_matches('{').trim_matches('}'))
            })
            .collect())
    } else {
        Ok(Vec::new())
    }
}