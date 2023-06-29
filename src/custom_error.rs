use super::captures::MultiCapture;
use super::*;

pub mod py_error {
    use super::*;

    pub fn make_error(
        custom_class_error: &PyObject,
        extra_with_value: Option<HashMap<&str, &str>>,
    ) -> PyResult<()> {
        Python::with_gil(|py| -> PyResult<()> {
            // Создаем объект класса ошибки с переданными параметрами
            let extra = types::PyDict::new(py);
            if let Some(extra_hm) = extra_with_value {
                for (key, value) in extra_hm {
                    extra.set_item(key, value)?;
                }
            }
            let custom_class_error = custom_class_error.downcast::<types::PyType>(py)?;
            custom_class_error.setattr(EXTRA_FROM_CLASS_PY, extra)?;
            Err(PyErr::new::<PyException, _>(
                custom_class_error.to_object(py),
            ))
        })
    }

    pub fn get_extra_from_class<'a>(
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
    pub fn filling_extra<'a>(
        extra_name: &Vec<&'a str>,
        captures: &MultiCapture<'a>,
    ) -> Option<HashMap<&'a str, &'a str>> {
        let mut extra_values: HashMap<&str, &str> = HashMap::new();
        match captures {
            MultiCapture::DefaultCaptures(captures) => {
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
            }
            MultiCapture::FancyCaptures(captures) => {
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
            }
        }
        if !extra_values.is_empty() {
            return Some(extra_values);
        }
        return None;
    }
}
