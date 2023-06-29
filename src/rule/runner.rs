use super::captures::MultiCapture;
use super::validate::actions_from_the_requirement::next_or_error;
use std::collections::VecDeque;

use super::*;
impl Rule {
    pub fn run(py: Python, text: &str, rule: &Rule, class_template: &PyObject) -> PyResult<()> {
        let mut stack = VecDeque::from([(rule, text)]);
        while let Some(stack_rule) = stack.pop_back() {
            let captures = MultiCapture::find_captures(stack_rule.0, stack_rule.1)?;
            dbg!(&captures);
            if next_or_error(py, class_template, stack_rule.0, &captures)? {
                let texts = captures.to_str_vec();
                // Если найден первый этап, проверяем до первой ошибки
                if let Some(rgxs_set) = &stack_rule.0.unchacked_get_rgx_set() {
                    texts
                        .iter()
                        .map(|text| {
                            dbg!(text);
                            let index = Rule::get_selected_rules(rgxs_set, text);
                            index
                                .iter()
                                .map(|id| {
                                    dbg!(&id);
                                    stack.push_back((
                                        &stack_rule.0.unchacked_get_rgx_vec()[*id],
                                        text,
                                    ));
                                })
                                .for_each(drop);
                            stack_rule
                                .0
                                .unchacked_get_rgx_vec()
                                .iter()
                                .map(|rule| {
                                    if !stack.contains(&(rule, &text)) {
                                        stack.push_back((rule, text));
                                    }
                                })
                                .for_each(drop);
                        })
                        .for_each(drop);
                }

                // Если первый этап пройден, проверяем самые сложные правила
                if let Some(f_r) = stack_rule.0.subrules.as_ref().unwrap().get_fancy_rgx_vec() {
                    texts
                        .iter()
                        .map(|text| {
                            f_r.iter()
                                .map(|rules| stack.push_back((rules, text)))
                                .for_each(drop);
                        })
                        .for_each(drop);
                }
            }
        }
        Ok(())
    }
}
