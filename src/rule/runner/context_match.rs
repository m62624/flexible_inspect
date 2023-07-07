use super::*;

impl Rule {
    pub fn all_rules_for_all_matches(stack: &mut VecDeque<(&Rule, CaptureData)>) -> NextStep {
        let mut temp_stack: VecDeque<(&Rule, CaptureData)> = VecDeque::new();
        while let Some(mut frame) = stack.pop_front() {
            match Self::next_or_data_for_error(frame.0, &mut frame.1) {
                NextStep::Go => {
                    if let Some(simple_rules) = &frame
                        .0
                        .content_unchecked()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .simple_rules
                    {
                        for text in frame.1.text_for_capture.iter() {
                            for index in Rule::get_selected_rules(&simple_rules.regex_set, text) {
                                let mut captures = CaptureData::find_captures(
                                    &simple_rules.all_rules[index],
                                    text,
                                );
                                if let NextStep::Error(value) = Self::next_or_data_for_error(
                                    &simple_rules.all_rules[index],
                                    &mut captures,
                                ) {
                                    return NextStep::Error(value);
                                }
                                temp_stack.push_back((&simple_rules.all_rules[index], captures));
                            }
                        }
                        stack.extend(temp_stack.drain(..));
                        for text in frame.1.text_for_capture.iter() {
                            for rule in simple_rules.all_rules.iter() {
                                let mut captures = CaptureData::find_captures(rule, text);
                                if !stack.iter().any(|&(r, _)| r == rule) {
                                    dbg!("то что не попало в regexset");
                                    dbg!(&rule.content_unchecked().str_with_type);
                                    if let NextStep::Error(value) =
                                        Self::next_or_data_for_error(rule, &mut captures)
                                    {
                                        return NextStep::Error(value);
                                    }
                                    temp_stack.push_back((rule, captures));
                                }
                            }
                        }
                    }
                    if let Some(complex_rules) = &frame
                        .0
                        .content_unchecked()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .complex_rules
                    {
                        for text in frame.1.text_for_capture.iter() {
                            for rule in complex_rules {
                                let mut captures = CaptureData::find_captures(rule, text);
                                if let NextStep::Error(value) =
                                    Self::next_or_data_for_error(rule, &mut captures)
                                {
                                    return NextStep::Error(value);
                                }
                                temp_stack.push_back((rule, captures));
                            }
                        }
                    }
                    stack.extend(temp_stack.drain(..));
                }
                NextStep::Finish => (),
                NextStep::Error(v) => return NextStep::Error(v),
            }
        }
        NextStep::Finish
    }
}
