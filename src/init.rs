use crate::*;
use ::regex::Regex;
use lazy_static::lazy_static;
use pyo3::exceptions;
use pyo3::exceptions::PyTypeError;
pub mod extra {
    use super::*;
    /// Получаем **extra переменные в виде коллекций
    pub fn extra_from_class<'a, T: AsRef<str> + Send + Sync>(
        class_template: &types::PyType,
        attr: T,
    ) -> PyResult<Option<Vec<String>>> {
        let attr_value = class_template.getattr(attr.as_ref())?.to_string();
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{.+?\}").unwrap();
        }
        if RE.is_match(&attr_value) {
            Ok(Some(
                RE.captures_iter(&attr_value)
                    .map(|cap| {
                        cap.get(0).map_or(String::new(), |m| {
                            m.as_str().trim_matches('{').trim_matches('}').to_string()
                        })
                    })
                    .collect(),
            ))
        } else {
            Ok(None)
        }
    }
}

pub mod regex {
    use super::*;
    pub fn collection_regex(
        hash_map: &mut HashMap<String, String>,
        rgx_set: &mut Vec<String>,
        error_class: &types::PyType,
        id_from_error_class: String,
    ) -> PyResult<()> {
        match error_class
            .getattr(RULES_FROM_CLASS_PY)?
            .downcast::<types::PyList>()
        {
            Ok(pylist) => Ok(pylist
                .iter()
                .map(|packed| {
                    Ok(match packed.extract::<String>() {
                        Ok(line_to_regex) => match Regex::new(&line_to_regex) {
                            Ok(_) => {
                                rgx_set.push(line_to_regex.to_owned());
                                hash_map.insert(line_to_regex, id_from_error_class.to_owned());
                            }
                            Err(_) => {
                                return Err(PyErr::new::<PyTypeError, _>(format!(
                                    "{} --- Invalid regular expression",
                                    packed
                                )));
                            }
                        },
                        Err(_) => {
                            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                                "'{}' must be a 'String'",
                                packed
                            )))
                        }
                    })
                })
                .collect::<Result<_, _>>()?),
            Err(_) => Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'List[ string, string... ]'",
                RULES_FROM_CLASS_PY
            ))),
        }
    }
}

pub mod bytes_utf8 {
    use super::*;
    pub fn convert(text_raw: &types::PyBytes) -> PyResult<String> {
        match str::from_utf8(text_raw.as_bytes()) {
            Ok(str) => Ok(str.to_string()),
            Err(_) => Err(PyErr::new::<PyTypeError, _>(
                "'text_raw' must be UTF-8 encoded",
            )),
        }
    }
}
