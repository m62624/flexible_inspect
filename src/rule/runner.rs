use super::captures::MultiCapture;
use super::validate::actions_from_the_requirement::next_or_error;

use super::*;
impl Rule {
    pub fn run(
        py: Python,
        text: &str,
        rule: &Rule,
        class_template: &PyObject,
    ) -> PyResult<Option<PyObject>> {
        let captures = MultiCapture::find_captures(rule, text)?;
        dbg!(&captures);
        let new_texts = captures.to_str_vec();
        dbg!(&new_texts);
        let mut next_step = false;
        let error = next_or_error(py, class_template, rule, captures, &mut next_step)?;
        if let Some(e) = error {
            return Ok(Some(e));
        } else if next_step {
            if let Some(regex_set) = Rule::get_selected_rules(
                rule.subrules.as_ref().unwrap().get_default_rgx_set(),
                rule.subrules.as_ref().unwrap().get_default_rgx_vec(),
                text,
            ) {
                let x = regex_set;
                x.iter()
                    .map(|sub_rule| {
                        new_texts
                            .iter()
                            .map(|txt| {
                                dbg!(&txt);
                                dbg!(&sub_rule);
                                Self::run(py, txt, sub_rule, class_template)?;
                                Ok(())
                            })
                            .collect::<PyResult<Vec<_>>>()?;
                        Ok(())
                    })
                    .collect::<PyResult<Vec<_>>>()?;
            } else {
                rule.subrules
                    .as_ref()
                    .unwrap()
                    .get_fancy_rgx_vec()
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|rule| {
                        new_texts
                            .iter()
                            .map(|text| Self::run(py, text, rule, class_template))
                            .collect::<PyResult<Vec<_>>>()?;
                        Ok(())
                    })
                    .collect::<PyResult<Vec<_>>>()?;
            }
        }

        Ok(None)
    }
}
/*
// Создаем коллецкию для очереди
    let mut stack = VecDeque::from([rule]);
    // Флаг для следующего шага
    let mut next_step = false;
    while let Some(rule_from_stack) = stack.pop_back() {
        let captures = MultiCapture::find_captures(rule, text)?;
        if let Some(error) = actions_from_the_requirement::next_or_error(
            py,
            class_template,
            rule,
            captures,
            &mut next_step,
        )? {
            return Ok(Some(error));
        } else if next_step {
            if let Some(sub_rules) = &rule_from_stack.get_op_subrules() {
                if let Some(selected_rules) = Rule::get_selected_rules(
                    sub_rules.get_default_rgx_set(),
                    sub_rules.get_default_rgx_vec(),
                    text,
                ) {

                } else {
                }
            }
        }
    }
 */
