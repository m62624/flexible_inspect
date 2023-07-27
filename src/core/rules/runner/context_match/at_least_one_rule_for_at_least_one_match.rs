use super::*;

/// in this mode, at least one rule must be passed for at least on match
pub fn at_least_one_rule_for_at_least_one_match<'a, R, C>(
    // this parameter is required for logs
    rule_ref: &R::RuleType,
    // get a unique stack of one root cmplx_rule, necessary to bypass the recursion constraint
    stack: &mut VecDeque<(&'a R::RuleType, CaptureData<C>)>,
) -> NextStep
where
    R: CalculateValueRules<'a, C> + Debug,
    C: PartialEq + Eq + Hash + Debug,
{
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
        match NextStep::next_or_finish_or_error(frame.0, &mut frame.1) {
            NextStep::Go => {
                // ============================= LOG =============================
                debug!(
                    "success, run subrules from the root rule `({}, {:#?})`",
                    rule_ref.get_str(),
                    rule_ref.get_requirement()
                );
                // ===============================================================
                // Stores the error, if any
                let mut err_value: Option<HashMap<String, String>> = None;
                // Status that we found one rule per one match
                let mut found_rule = false;
                /*
                The first step is to get a RegexSet for each match, based on it,
                we get those rules that will definitely work, then check their modifiers
                 */
                'skip_data: for data in &frame.1.text_for_capture {
                    if let Some(simple_rules) = frame.0.get_simple_rules() {
                        let mut selected_rules = HashSet::new();
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
                            let mut captures = R::find_captures(rule_from_regexset, data);
                            if let NextStep::Error(error) =
                                NextStep::next_or_finish_or_error(rule_from_regexset, &mut captures)
                            {
                                // ============================= LOG =============================
                                trace!("the rule `{}` failed condition for data `{:#?}` ( this rule is categorized as `not in RegexSet` )", rule_from_regexset.get_str(), data );
                                // ===============================================================

                                err_value = error;
                                continue 'skip_this_rule;
                            }

                            // ============================= LOG =============================
                            trace!(
                                "found one rule `({}, {:#?})` for on match `{:#?}`",
                                rule_from_regexset.get_str(),
                                rule_from_regexset.get_requirement(),
                                data
                            );
                            // ===============================================================
                            found_rule = true;
                            selected_rules.insert(rule_from_regexset);
                            stack.push_back((rule_from_regexset, captures));
                            break 'skip_data;
                        }
                        // The second step, in this stage we go through those rules and matches that are not in `RegexSet`.
                        'not_in_regexset: for rule in simple_rules.0 {
                            if !selected_rules.contains(rule) {
                                // ============================= LOG =============================
                                trace!(
                                    "the rule `({}, {:#?})` is not in `RegexSet`",
                                    rule.get_str(),
                                    rule.get_requirement(),
                                );
                                // ===============================================================
                                let mut captures = R::find_captures(rule, data);
                                if let NextStep::Error(err) =
                                    NextStep::next_or_finish_or_error(rule, &mut captures)
                                {
                                    err_value = err;
                                    continue 'not_in_regexset;
                                }

                                // ============================= LOG =============================
                                info!(
                                    "found one rule `({}, {:#?})` for match `{:#?}`",
                                    rule.get_str(),
                                    rule.get_requirement(),
                                    data
                                );
                                // ===============================================================
                                found_rule = true;
                                stack.push_back((rule, captures));
                                break 'skip_data;
                            }
                        }
                    }
                    // The hird step, bypass the rules with the Lookahead and Lookbehind regex.
                    if let Some(cmplx_rules) = frame.0.get_complex_rules() {
                        if !found_rule {
                            'skip_this_cmplx_rule: for rule in cmplx_rules {
                                // ============================= LOG =============================
                                trace!(
                                    "the rule `({}, {:#?})` from `complex_rules`",
                                    rule.get_str(),
                                    rule.get_requirement()
                                );
                                // ===============================================================
                                let mut captures = R::find_captures(rule, data);
                                if let NextStep::Error(err) =
                                    NextStep::next_or_finish_or_error(rule, &mut captures)
                                {
                                    err_value = err;
                                    continue 'skip_this_cmplx_rule;
                                }
                                // ============================= LOG =============================
                                info!(
                                    "found one rule `({}, {:#?})` for match `{:#?}`",
                                    rule.get_str(),
                                    rule.get_requirement(),
                                    data
                                );
                                // ===============================================================
                                found_rule = true;
                                stack.push_back((rule, captures));
                                break 'skip_data;
                            }
                        }
                    }
                }
                if !found_rule {
                    // ================= (LOG) =================
                    error!("no rules were found for any of the matches");
                    // =========================================
                    return NextStep::Error(err_value);
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
                // ================= (LOG) =================
                error!("no rules were found for any of the matches");
                // =========================================
                return NextStep::Error(err);
            }
        }
    }

    NextStep::Finish
}
