use super::*;
use std::collections::VecDeque;
#[pymethods]
impl TemplateValidator {
    fn validate(&self, text: &types::PyBytes) -> PyResult<()> {
        let text = bytes_to_string_utf8(text.as_bytes())?;
        self.exceptions
            .iter()
            .map(|exception_container| Self::step_by_step_on_the_class(exception_container, &text))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(())
    }
}

impl TemplateValidator {
    pub fn step_by_step_on_the_class(
        exception_class: &ExceptionContainer,
        text: &str,
    ) -> PyResult<()> {
        if let Some(selected_rules) = exception_class.get_selected_rules(&text) {
            selected_rules
                .iter()
                .try_for_each(|rule| Self::step_by_step_one_the_rule(rule, text))?;
        }
        exception_class
            .get_fancy_rules()
            .iter()
            .try_for_each(|rule| Self::step_by_step_one_the_rule(rule, text))?;
        Ok(())
    }
    pub fn step_by_step_one_the_rule(rule: &rule::Rule, text: &str) -> PyResult<()> {
        let mut stack: VecDeque<&rule::Rule> = VecDeque::new();
        let mut error_flag = false;
        Ok(())
    }
}

pub fn bytes_to_string_utf8(bytes: &[u8]) -> PyResult<String> {
    match String::from_utf8(bytes.into()) {
        Ok(result) => Ok(result),
        Err(_) => Err(PyErr::new::<exceptions::PyValueError, _>(format!(
            "{:#?} is not a valid UTF-8 string",
            bytes
        ))),
    }
}
