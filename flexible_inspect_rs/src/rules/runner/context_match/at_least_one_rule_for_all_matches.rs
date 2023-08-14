use super::*;

/// in this mode, at least one rule must be passed for all matches
pub fn at_least_one_rule_for_all_matches<'a, R, C>(
    // get a unique stack of one root cmplx_rule, necessary to bypass the recursion constraint
    stack: &mut VecDeque<(&'a R::RuleType, CaptureData<C>)>,
) -> NextStep
where
    R: CalculateValueRules<'a, C> + Debug,
    C: PartialEq + Eq + Hash + Debug,
{
    let mut temp_stack = Some(VecDeque::new());
    if let Some(mut frame) = stack.pop_front() {
        trace!(
            "deleted rule from unique stack: ({}, {})",
            frame.0.get_str().yellow(),
            format!("{:#?}", frame.0.get_requirement()).yellow()
        );
        let mut counter_one_rule = HashMap::new();
        match NextStep::next_or_finish_or_error(frame.0, &mut frame.1) {
            NextStep::Go => {
                // ============================= LOG =============================
                debug!(
                    "run subrules from the root rule `({}, {})`",
                    frame.0.get_str().yellow(),
                    format!("{:#?}", frame.0.get_requirement()).yellow()
                );
                // ===============================================================
                // Status, whether we found one rule for all matches
                let mut one_rule_found = false;
                // Stores the error, if any
                let mut error_value: Option<HashMap<String, String>> = None;
                let mut selected_rules = HashSet::new();
                if let Some(simple_rules) = &frame.0.get_simple_rules() {
                    /*
                    The first step is to get a RegexSet for each match, based on it,
                    we get those rules that will definitely work, then check their modifiers
                     */
                    'skip_data: for data in &frame.1.text_for_capture {
                        'skip_this_rule: for index in R::get_selected_rules(simple_rules.1, data) {
                            let rule_from_regexset = simple_rules.0.get_index(index).unwrap();
                            // ============================= LOG =============================
                            debug!(
                                "found the rule `({}, {})` (root rule `({}, {})`) from the `RegexSet` category\nfor data`{:#?}`",
                                rule_from_regexset.get_str(),
                                format!("{:#?}", rule_from_regexset.get_requirement()).yellow(),
                                frame.0.get_str().yellow(),
                                format!("{:#?}", frame.0.get_requirement()).yellow(),
                                data
                            );
                            // ===============================================================
                            *counter_one_rule.entry(index).or_insert(0) += 1;
                            if counter_one_rule[&index] == frame.1.text_for_capture.len() {
                                let mut captures = R::find_captures(rule_from_regexset, data);
                                if let NextStep::Error(err) = NextStep::next_or_finish_or_error(
                                    rule_from_regexset,
                                    &mut captures,
                                ) {
                                    // ============================= LOG =============================
                                    debug!(
                                    "the rule `({}, {})` (root rule `({}, {})`) failed condition\nfor data (this rule is categorized as `not in RegexSet`) `{:#?}` ",
                                    rule_from_regexset.get_str(),
                                    format!("{:#?}", rule_from_regexset.get_requirement()).yellow(),    
                                    frame.0.get_str().yellow(),
                                    format!("{:#?}", frame.0.get_requirement()).yellow(),
                                    data
                                );
                                    // ===============================================================

                                    error_value = err;
                                    continue 'skip_this_rule;
                                }
                                // ============================= LOG =============================
                                info!(
                                    "the rule `({}, {})` (root rule `({}, {})`) worked successfully for all matches (`RegexSet`)",
                                    rule_from_regexset.get_str().yellow(),
                                    format!("{:#?}", rule_from_regexset.get_requirement()).yellow(),
                                    frame.0.get_str().yellow(),
                                    format!("{:#?}", frame.0.get_requirement()).yellow(),
                                );
                                // ===============================================================
                                one_rule_found = true;
                                selected_rules.insert(rule_from_regexset);
                                if let NextStep::Go = NextStep::next_or_finish_or_error(
                                    rule_from_regexset,
                                    &mut captures,
                                ) {
                                    if let Some(temp_stack) = temp_stack.as_mut() {
                                        temp_stack.push_back((rule_from_regexset, captures));
                                    }
                                } else {
                                    stack.push_back((rule_from_regexset, captures));
                                    temp_stack = None;
                                    break 'skip_data;
                                }
                            }
                        }
                    }
                    counter_one_rule.clear();
                    if !one_rule_found {
                        // The second step, in this stage we go through those rules and matches that are not in `RegexSet`.
                        'not_in_regexset: for rule in simple_rules.0 {
                            if !selected_rules.contains(rule) {
                                if let Some(data) = frame.1.text_for_capture.iter().next() {
                                    let mut captures = R::find_captures(rule, data);
                                    if let NextStep::Error(err) =
                                        NextStep::next_or_finish_or_error(rule, &mut captures)
                                    {
                                        error_value = err;
                                        continue 'not_in_regexset;
                                    }

                                    // ============================= LOG =============================
                                    info!(
                                        "found one rule for all matches: ({}, {:#?}) (root rule `({}, {})`) ",
                                        rule.get_str().yellow(),
                                        format!("{:#?}", rule.get_requirement()).yellow(),
                                        frame.0.get_str().yellow(),
                                        format!("{:#?}", frame.0.get_requirement()).yellow(),
                                    );
                                    // ===============================================================
                                    if let NextStep::Go =
                                        NextStep::next_or_finish_or_error(rule, &mut captures)
                                    {
                                        if let Some(temp_stack) = temp_stack.as_mut() {
                                            temp_stack.push_back((rule, captures));
                                        }
                                    } else {
                                        stack.push_back((rule, captures));
                                        temp_stack = None;
                                    }
                                }
                                one_rule_found = true;
                            }
                        }
                    }
                }
                // The hird step, bypass the rules with the Lookahead and Lookbehind regex.
                if let Some(cmplx_rules) = frame.0.get_complex_rules() {
                    if !one_rule_found {
                        'skip_this_cmplx_rule: for rule in cmplx_rules {
                            for data in &frame.1.text_for_capture {
                                let mut captures = R::find_captures(rule, data);
                                if let NextStep::Error(err) =
                                    NextStep::next_or_finish_or_error(rule, &mut captures)
                                {
                                    error_value = err;
                                    continue 'skip_this_cmplx_rule;
                                }
                                // ============================= LOG =============================
                                info!(
                                    "found one rule for all matches: `({}, {})` (root rule `({}, {})`) from the `Complex Rule` category",
                                    rule.get_str().yellow(),
                                    format!("{:#?}", rule.get_requirement()).yellow(),
                                    frame.0.get_str().yellow(),
                                    format!("{:#?}", frame.0.get_requirement()).yellow(),
                                );
                                // ===============================================================
                                if let NextStep::Go =
                                    NextStep::next_or_finish_or_error(rule, &mut captures)
                                {
                                    if let Some(temp_stack) = temp_stack.as_mut() {
                                        temp_stack.push_back((rule, captures));
                                    }
                                } else {
                                    stack.push_back((rule, captures));
                                    temp_stack = None;
                                }
                            }
                            one_rule_found = true;
                        }
                    }
                }
                if one_rule_found {
                    if let Some(temp_stack) = temp_stack.as_mut() {
                        stack.extend(temp_stack.drain(..));
                    }
                } else {
                    // ============================= LOG =============================
                    error!("not found one rule for all matches");
                    // ===============================================================

                    // Если мы не нашли правило, то возвращаем ошибку
                    return NextStep::Error(error_value);
                }
            }
            NextStep::Finish => {
                // ============================= LOG =============================
                debug!(
                    "the rule `({}, {})` is finished, the result is `Ok`",
                    frame.0.get_str().yellow(),
                    format!("{:#?}", frame.0.get_requirement()).yellow()
                );
                // ===============================================================
            }
            NextStep::Error(err) => {
                // ============================= LOG =============================
                error!("not found one rule for all matches");
                // ===============================================================
                return NextStep::Error(err);
            }
        }
    }
    NextStep::Finish
}
