use super::captures::CaptureData;
use super::next::NextStep;
use super::*;
use std::collections::VecDeque;

impl Rule {
    pub fn run(rule: &Rule, text: &str) -> NextStep {
        let mut stack = VecDeque::from([(rule, text)]);
        while let Some(stack_rule) = stack.pop_front() {
            let mut captures = CaptureData::find_captures(stack_rule.0, stack_rule.1);
            match Self::next_or_data_for_error(stack_rule.0, &mut captures) {
                NextStep::Go => {
                    if let Some(simple_rules) = &stack_rule
                        .0
                        .content_unchecked()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .simple_rules
                    {
                        captures.text_for_capture.iter().for_each(|txt| {
                            Rule::get_selected_rules(&simple_rules.regex_set, txt)
                                .iter()
                                .for_each(|index| {
                                    stack.push_back((&simple_rules.all_rules[*index], txt))
                                });
                        });
                        captures.text_for_capture.iter().for_each(|txt| {
                            simple_rules.all_rules.iter().for_each(|rule| {
                                if !stack.contains(&(rule, *txt)) {
                                    stack.push_back((rule, txt));
                                }
                            });
                        });
                    }
                    if let Some(complex_rules) = &stack_rule
                        .0
                        .content_unchecked()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .complex_rules
                    {
                        captures.text_for_capture.iter().for_each(|txt| {
                            complex_rules
                                .iter()
                                .for_each(|rule| stack.push_back((rule, txt)))
                        });
                    }
                }
                NextStep::Finish => (),
                NextStep::Error(value) => return NextStep::Error(value),
            }
        }
        NextStep::Finish
    }
}
