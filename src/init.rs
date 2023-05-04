use crate::*;
use ::regex::Regex;
use futures::try_join;
use lazy_static::lazy_static;
use pyo3::exceptions;
use pyo3::exceptions::PyTypeError;
pub mod extra {
    use super::*;
    /// Получаем **extra переменные в виде коллекций
    pub async fn extra_from_class<'a, T: AsRef<str> + Send + Sync>(
        class_template: &types::PyType,
        attr: T,
    ) -> PyResult<Option<Box<dyn Iterator<Item = String> + 'a>>> {
        let attr_value = class_template.getattr(attr.as_ref())?.to_string();
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{.+?\}").unwrap();
        }
        if RE.is_match(&attr_value) {
            Ok(Some(Box::new(
                RE.captures_iter(&attr_value)
                    .map(|cap| {
                        cap.get(0).map_or(String::new(), |m| {
                            m.as_str().trim_matches('{').trim_matches('}').to_string()
                        })
                    })
                    .collect::<Vec<_>>()
                    .into_iter(),
            )))
        } else {
            Ok(None)
        }
    }
}

pub mod regex {

    use super::*;

    pub fn check_regex<T: AsRef<str> + Send + Sync>(regex_template: T) -> PyResult<Regex> {
        match Regex::new(regex_template.as_ref()) {
            Ok(value) => return Ok(value),
            Err(_) => Err(PyErr::new::<PyTypeError, _>("Invalid regular expression")),
        }
    }
    pub async fn regex_from_class<'a>(
        class_template: &'a types::PyType,
    ) -> PyResult<Box<dyn Iterator<Item = Regex> + 'a>> {
        match class_template
            .getattr(RULES_FROM_CLASS_PY)?
            .downcast::<types::PyList>()
        {
            Ok(pylist) => Ok(Box::new(
                pylist
                    .iter()
                    .map(|packed| match packed.extract::<String>() {
                        Ok(s) => check_regex(s),
                        Err(_) => Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                            "'{}' must be a 'String'",
                            packed
                        ))),
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter(),
            )),
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
pub mod data {
    use super::*;
    pub fn multi_data<'a>(
        one_error: &'a types::PyType,
    ) -> PyResult<(
        Box<dyn Iterator<Item = Regex> + 'a>,
        Option<Box<dyn Iterator<Item = String> + 'a>>,
    )> {
        async_std::task::block_on(async {
            try_join!(
                regex::regex_from_class(&one_error),
                extra::extra_from_class(&one_error, MESSAGE_WITH_EXTRA_FROM_CLASS_PY)
            )
        })
    }
}
