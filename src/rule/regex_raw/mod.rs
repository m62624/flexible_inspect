mod getters;
use pyo3::exceptions;

use super::*;
#[derive(Debug, Clone)]
/// --> Rule
pub enum RegexRaw {
    DefaultR(Box<str>),
    FancyR(Box<str>),
}

impl RegexRaw {
    pub fn new(pattern: String) -> PyResult<RegexRaw> {
        if regex::Regex::new(&pattern).is_ok() {
            return Ok(RegexRaw::DefaultR(pattern.into_boxed_str()));
        } else if fancy_regex::Regex::new(&pattern).is_ok() {
            return Ok(RegexRaw::FancyR(pattern.into_boxed_str()));
        }
        return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "Expected `Regex` or `FancyRegex`, got `{}`",
            pattern
        )));
    }
}
