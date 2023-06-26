use super::*;
use crate::template_validator::captures::MultiCapture;
use std::collections::VecDeque;
pub mod single_work {
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
            if let Err(e) = step_by_step_in_the_rule(py, rule, exception_class.get_obj(), text) {
                errors.push(e.to_object(py));
                break;
            }
        }
        Ok(())
    }
    pub fn step_by_step_in_the_rule(
        py: Python,
        rule: &rule::Rule,
        obj: &PyObject,
        text: &str,
    ) -> PyResult<()> {
        let mut stack: VecDeque<&Rule> = VecDeque::new();
        let mut next_step = false;
        stack.push_back(rule);
        while let Some(rule_stack) = stack.pop_back() {
            let captures = MultiCapture::find_captures(rule_stack, text)?;
            next_step::result_on_the_match(py, rule_stack, obj, captures, &mut next_step)?;
        }
        if next_step {
            if let Some(subrules) = rule.get_regex_set(text) {
                subrules
                    .iter()
                    .map(|rule| {
                        stack.push_back(rule);
                    })
                    .for_each(drop);
            } else {
                if let Some(subrules) = rule.get_op_subrules() {
                    subrules
                        .iter()
                        .map(|rule| {
                            stack.push_back(rule);
                        })
                        .for_each(drop);
                }
            }
        }
        Ok(())
    }
}

pub mod multi_work {}
