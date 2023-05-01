use crate::*;
use lazy_static::lazy_static;
use pyo3::exceptions::PyTypeError;
use pyo3::types::PyType;
use regex::Regex;
/// Получаем строку и возвращаем регулярное вырарежние
pub fn get_regex<T: AsRef<str> + Send + Sync>(regex_template: T) -> PyResult<Regex> {
    match Regex::new(regex_template.as_ref()) {
        Ok(value) => return Ok(value),
        Err(_) => Err(PyErr::new::<PyTypeError, _>("Invalid regular expression")),
    }
}
pub fn regex_validate<'py>(raw_data: &'py [u8], regex: Regex) {}
/*
    `get_extra`
    так как мы получаем сам класс а не его экземлпяр, мы не можем узнать какие extra
    значения присутствуют, но каждый класс ошибки должен реализовать поле 'template'.
    Тем самым, с помощью regex выражений, мы можем получить все переменные, которые
    и будут служить **extra. Если `template` не инициализирован,
    то происходит проброс ошибки `NameError`
*/
/// Получаем **extra переменные в виде коллекций
pub fn get_extra<T: AsRef<str> + Send + Sync>(obj: &PyType, attr: T) -> PyResult<Vec<String>> {
    let attr_value = obj.getattr(attr.as_ref())?.to_string();
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
            "**extra is not defined (from `template`)",
        )));
    }
}
