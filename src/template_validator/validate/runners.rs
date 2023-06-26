use super::*;
pub mod sync_work {
    use super::*;
    pub fn step_by_step_in_the_class(
        py: Python<'_>,
        errors: &mut Vec<PyObject>,
        exception_class: &ExceptionContainer,
        text: &str,
    ) -> PyResult<()> {
        let rules_iter: Box<dyn Iterator<Item = &Rule>> =
            if let Some(selected_rules) = exception_class.get_selected_rules(&text) {
                Box::new(selected_rules.into_iter())
            } else {
                Box::new(exception_class.get_fancy_rules().into_iter())
            };

        for rule in rules_iter {
            if let Err(e) = step_by_step_in_the_rule(rule, text) {
                errors.push(e.to_object(py));
                break;
            }
        }
        Ok(())
    }
    pub fn step_by_step_in_the_rule(rule: &rule::Rule, text: &str) -> PyResult<()> {
        todo!()
    }
}

pub mod async_work {}
