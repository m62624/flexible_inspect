use crate::core::rules::next::NextStep;
use crate::core::rules::traits::{CalculateValueRules, RuleBase};
use crate::core::rules::CaptureData;
use indexmap::IndexSet;
use log::{debug, error, info, trace};
use std::collections::{HashMap, HashSet};
use std::{collections::VecDeque, fmt::Debug, hash::Hash};

/// in this mode, a rule must be passed for each match
pub fn all_rules_for_all_matches<'a, R, C>(
    // this parameter is required for logs
    rule_ref: &R::RuleType,
    // get a unique stack of one root rule, necessary to bypass the recursion constraint
    stack: &mut VecDeque<(&R::RuleType, CaptureData<C>)>,
) -> NextStep
where
    R: CalculateValueRules<'a, C> + Debug,
    C: PartialEq + Eq + Hash + Debug,
{
    // ============================= LOG =============================
    debug!("the local rule stack `{}` is received", {
        rule_ref.get_str()
    });
    // ===============================================================
    let mut temp_stack: VecDeque<(&R::RuleType, CaptureData<C>)> = VecDeque::new();
    while let Some(mut frame) = stack.pop_front() {
        // ============================= LOG =============================
        debug!(
            "\ncheck the state of the rule `({}, {:#?})` \nfrom the local stack `{}`",
            frame.0.get_str(),
            frame.0.get_requirement(),
            rule_ref.get_str()
        );
        // ===============================================================
        match NextStep::next_or_finish_or_error(frame.0, &mut frame.1) {
            NextStep::Go => {
                if let Some(simple_rules) = &frame.0.get_simple_rules() {
                    let mut counter_of_each_rule = HashMap::new();
                    let mut selected_text = HashMap::new();
                    let mut selected_rules: HashSet<_> = HashSet::new();
                    // ============================= LOG =============================
                    debug!(
                        "success, run subrules from the root rule `({}, {:#?})`",
                        rule_ref.get_str(),
                        rule_ref.get_requirement()
                    );
                    // ===============================================================

                    /*
                    The first step is to get a RegexSet for each match, based on it,
                    we get those rules that will definitely work, then check their modifiers
                     */
                    for data in &frame.1.text_for_capture {
                        for index in R::get_selected_rules(simple_rules.1, data) {
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
                                error!(
                                    "the rule `{}` failed condition for data `{:#?}`",
                                    rule_from_regexset.get_str(),
                                    data
                                );
                                // ===============================================================
                                return NextStep::Error(error);
                            }
                            /*
                            For each rule, let's mark the data that has already been checked,
                            so that we can exclude it in the second cycle
                             */
                            selected_text
                                .entry(rule_from_regexset)
                                .or_insert_with(HashSet::new)
                                .insert(data);
                            /*
                            Since in this mode `rule` * `data`, where each rule should work for every match,
                            we check how many times one rule from regexset was passed for matches,
                            if a rule worked for all matches, we write it to an exception.
                             */
                            *counter_of_each_rule.entry(index).or_insert(0) += 1;
                            if counter_of_each_rule[&index] == frame.1.text_for_capture.len() {
                                // ============================= LOG =============================
                                trace!(
                                    "the ({}, {:#?}) rule worked successfully for all matches",
                                    rule_from_regexset.get_str(),
                                    rule_from_regexset.get_requirement(),
                                );
                                // ===============================================================
                                selected_rules.insert(rule_from_regexset);
                                temp_stack.push_back((rule_from_regexset, captures));
                            }
                        }
                    }
                    // The second step, in this stage we go through those rules and matches that are not in `RegexSet`.
                    for data in &frame.1.text_for_capture {
                        // we go through all the simple rules
                        for rule in simple_rules.0 {
                            // So the first condition is that we exclude those rules
                            // that have already been processed in RegexSet.
                            // ( excluded only those who, for all the coincidence, have been successful )
                            if !selected_rules.contains(rule) {
                                // If this rule worked for several matches,
                                // but not for all of them, then we get those values that have already been processed
                                // and exclude them
                                if let Some(value) = selected_text.get(rule) {
                                    if !value.contains(data) {
                                        if let NextStep::Error(err) =
                                            not_in_regexset::<R, C>(rule, data)
                                        {
                                            return NextStep::Error(err);
                                        }
                                    }
                                } else {
                                    // If there were no successful matches in this rule,
                                    // it means that this is the first time
                                    // this rule has been run for validation
                                    if let NextStep::Error(err) =
                                        not_in_regexset::<R, C>(rule, data)
                                    {
                                        return NextStep::Error(err);
                                    }
                                }
                            }
                        }
                    }
                }
                // The hird step, bypass the rules with the Lookahead and Lookbehind regex.
                if let Some(complex_rules) = frame.0.get_complex_rules() {
                    for data in &frame.1.text_for_capture {
                        for cmplx_rule in complex_rules {
                            let mut captures = R::find_captures(cmplx_rule, data);
                            if let NextStep::Error(error) =
                                NextStep::next_or_finish_or_error(cmplx_rule, &mut captures)
                            {
                                // ============================= LOG =============================
                                error!(
                                    "the rule `{}` failed condition for data `{:#?}`",
                                    cmplx_rule.get_str(),
                                    data
                                );
                                // ===============================================================
                                return NextStep::Error(error);
                            }
                        }
                    }
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
            NextStep::Error(_) => {}
        }
    }
    NextStep::Finish
}

// Function for checking rules not included in `RegexSet`.
fn not_in_regexset<'a, R, C>(rule: &R::RuleType, data: &C) -> NextStep
where
    R: CalculateValueRules<'a, C> + Debug,
    C: PartialEq + Eq + Hash + Debug,
{
    // ============================= LOG =============================
    trace!(
        "the rule `({}, {:#?})` is not in `RegexSet` for data `{:#?}`",
        rule.get_str(),
        rule.get_requirement(),
        data
    );
    // ===============================================================
    let mut captures = R::find_captures(rule, data);
    if let NextStep::Error(error) = NextStep::next_or_finish_or_error(rule, &mut captures) {
        // ============================= LOG =============================
        error!(
            "the rule `{}` failed condition for data `{:#?}`",
            rule.get_str(),
            data
        );
        return NextStep::Error(error);
        // ===============================================================
    }
    NextStep::Finish
}
