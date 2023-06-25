use super::*;
#[pymethods]
impl TemplateValidator {
    fn validate(&self, text: &types::PyBytes) -> PyResult<()> {
        let text = bytes_to_string_utf8(text.as_bytes())?;
        let err = self
            .exceptions
            .iter()
            .map(|exception_container| {
                if let Some(selected_rules) = exception_container.get_selected_rules(&text) {
                    selected_rules
                        .iter()
                        .map(|rule_root| {
                            //validator_core(Vec<&Rule>)

                            Ok(())
                        })
                        .collect::<PyResult<Vec<_>>>()?;
                }
                exception_container
                    .get_fancy_rules()
                    .iter()
                    .map(|rule| {
                        // validator_core(FancyRegex)
                        Ok(())
                    })
                    .collect::<PyResult<Vec<_>>>()?;
                Ok(())
            })
            .collect::<PyResult<Vec<_>>>()?;
        Ok(())
    }
}

impl TemplateValidator {
    pub fn step_by_step_on_the_rule(rule: &rule::Rule) -> PyResult<()> {
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
