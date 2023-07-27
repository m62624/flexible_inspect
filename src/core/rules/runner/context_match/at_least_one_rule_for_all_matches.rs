use crate::core::rules::traits::RuleBase;
use crate::core::rules::{next::NextStep, traits::CalculateValueRules, CaptureData};
use log::{debug, error, info, trace};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

/// in this mode, at least one rule must be passed for all matches
pub fn at_least_one_rule_for_all_matches<'a, R, C>(
    // this parameter is required for logs
    rule_ref: &R::RuleType,
    // get a unique stack of one root cmplx_rule, necessary to bypass the recursion constraint
    stack: &mut VecDeque<(&'a R::RuleType, CaptureData<C>)>,
) -> NextStep
where
    R: CalculateValueRules<'a, C> + Debug,
    C: PartialEq + Eq + Hash + Debug,
{
    let mut temp_stack: VecDeque<(&R::RuleType, CaptureData<C>)> = VecDeque::new();
    while let Some(mut frame) = stack.pop_front() {
        // ============================= LOG =============================
        debug!(
            "\ncheck the state of the rule `({}, {:#?})` \nfrom the local stack `({}, {:#?})`",
            frame.0.get_str(),
            frame.0.get_requirement(),
            rule_ref.get_str(),
            rule_ref.get_requirement()
        );
        // ===============================================================
        let mut counter_one_rule = HashMap::new();
        match NextStep::next_or_finish_or_error(frame.0, &mut frame.1) {
            NextStep::Go => {
                // Статус, нашли ли мы одно правило для всех совпадений
                let mut one_rule_found = false;
                // Хранит ошибку, если она есть
                let mut error_value: Option<HashMap<String, String>> = None;
                let mut selected_rules = HashSet::new();
                if let Some(simple_rules) = &frame.0.get_simple_rules() {
                    'skip_data: for data in &frame.1.text_for_capture {
                        'skip_this_rule: for index in R::get_selected_rules(simple_rules.1, data) {
                            let rule_from_regexset = simple_rules.0.get_index(index).unwrap();
                            // ============================= LOG =============================
                            trace!(
                                "found `({}, {:#?})` rule from `RegexSet` for `{:#?}` data",
                                rule_from_regexset.get_str(),
                                rule_from_regexset.get_requirement(),
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
                                    error_value = err;
                                    continue 'skip_this_rule;
                                }
                                // ============================= LOG =============================
                                info!(
                                    "found one rule for all matches: ({}, {:#?})",
                                    rule_from_regexset.get_str(),
                                    rule_from_regexset.get_requirement()
                                );
                                // ===============================================================
                                one_rule_found = true;
                                selected_rules.insert(rule_from_regexset);
                                temp_stack.push_back((rule_from_regexset, captures));
                                break 'skip_data;
                            }
                        }
                    }
                    counter_one_rule.clear();
                    if !one_rule_found {
                        'not_in_regexset: for rule in simple_rules.0 {
                            if !selected_rules.contains(rule) {
                                // ============================= LOG =============================
                                trace!(
                                    "the rule `({}, {:#?})` is not in `RegexSet`",
                                    rule.get_str(),
                                    rule.get_requirement(),
                                );
                                // ===============================================================
                                'skip_this_rule_not_in_regexset: for data in
                                    &frame.1.text_for_capture
                                {
                                    let mut captures = R::find_captures(rule, data);
                                    if let NextStep::Error(err) =
                                        NextStep::next_or_finish_or_error(rule, &mut captures)
                                    {
                                        error_value = err;
                                        continue 'not_in_regexset;
                                    }

                                    // ============================= LOG =============================
                                    info!(
                                        "found one rule for all matches: ({}, {:#?})",
                                        rule.get_str(),
                                        rule.get_requirement()
                                    );
                                    // ===============================================================
                                    temp_stack.push_back((rule, captures));
                                    break 'skip_this_rule_not_in_regexset;
                                }
                                one_rule_found = true;
                            }
                        }
                    }
                }
                if let Some(cmplx_rules) = frame.0.get_complex_rules() {
                    if !one_rule_found {
                        'skip_this_cmplx_rule: for rule in cmplx_rules {
                            // ============================= LOG =============================
                            trace!(
                                "the rule `({}, {:#?})` from `complex_rules`",
                                rule.get_str(),
                                rule.get_requirement()
                            );
                            // ===============================================================
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
                                    "found one rule for all matches: ({}, {:#?})",
                                    rule.get_str(),
                                    rule.get_requirement()
                                );
                                // ===============================================================
                                temp_stack.push_back((rule, captures));
                            }
                            one_rule_found = true;
                        }
                    }
                }
                if one_rule_found {
                    stack.extend(temp_stack.drain(..));
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
                    "the rule `({}, {:#?})` is finished, the result is `Ok`",
                    frame.0.get_str(),
                    frame.0.get_requirement()
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
