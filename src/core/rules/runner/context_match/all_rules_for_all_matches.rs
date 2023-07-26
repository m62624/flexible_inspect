use crate::core::rules::next::NextStep;
use crate::core::rules::traits::{CalculateValueRules, RuleBase};
use crate::core::rules::CaptureData;
use indexmap::IndexSet;
use log::{debug, error, info, trace};
use std::collections::{HashMap, HashSet};
use std::{collections::VecDeque, fmt::Debug, hash::Hash};

pub fn all_rules_for_all_matches<'a, R, C>(
    rule_ref: &R::RuleType,
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
                    let mut counter_of_each_rule: HashMap<usize, usize> = HashMap::new();
                    let mut selected_text: HashMap<
                        &<R as CalculateValueRules<C>>::RuleType,
                        HashSet<&C>,
                    > = HashMap::new();
                    let mut selected_rules: HashSet<_> = HashSet::new();
                    // ============================= LOG =============================
                    debug!(
                        "success, run subrules from the root rule `({}, {:#?})`",
                        rule_ref.get_str(),
                        rule_ref.get_requirement()
                    );
                    // ===============================================================
                    for text in &frame.1.text_for_capture {
                        for index in R::get_selected_rules(simple_rules.1, text) {
                            let rule_from_regexset = simple_rules.0.get_index(index).unwrap();
                            // ============================= LOG =============================
                            trace!(
                                "found `({}, {:#?})` rule from `RegexSet` for `{:#?}` data",
                                rule_from_regexset.get_str(),
                                rule_from_regexset.get_requirement(),
                                text
                            );
                            // ===============================================================
                            let mut captures = R::find_captures(rule_from_regexset, text);
                            if let NextStep::Error(error) =
                                NextStep::next_or_finish_or_error(rule_from_regexset, &mut captures)
                            {
                                // ============================= LOG =============================
                                error!(
                                    "the rule `{}` failed condition for data `{:#?}`",
                                    rule_from_regexset.get_str(),
                                    text
                                );
                                // ===============================================================
                                return NextStep::Error(error);
                            }
                            selected_text
                                .entry(rule_from_regexset)
                                .or_insert_with(HashSet::new)
                                .insert(text);

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

                    for text in &frame.1.text_for_capture {
                        for rule in simple_rules.0 {
                            if !selected_rules.contains(rule) {
                                if let Some(value) = selected_text.get(rule) {
                                    if !value.contains(text) {
                                        if let NextStep::Error(err) =
                                            not_in_regeset::<R, C>(rule, text)
                                        {
                                            return NextStep::Error(err);
                                        }
                                    }
                                } else {
                                    if let NextStep::Error(err) = not_in_regeset::<R, C>(rule, text)
                                    {
                                        return NextStep::Error(err);
                                    }
                                }
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

fn not_in_regeset<'a, R, C>(rule: &R::RuleType, data: &C) -> NextStep
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
