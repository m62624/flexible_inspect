use crate::core::rules::next::NextStep;
use crate::core::rules::traits::{CalculateValueRules, RuleBase};
use crate::core::rules::CaptureData;
use log::{debug, error, info, trace};
use std::collections::HashMap;
use std::fmt::Display;
use std::{collections::VecDeque, fmt::Debug, hash::Hash};

pub fn all_rules_for_all_matches<'a, R, C>(
    rule_ref: &R::RuleType,
    stack: &mut VecDeque<(&R::RuleType, CaptureData<C>)>,
) -> NextStep
where
    C: PartialEq + Eq + Hash + Debug,
    R: CalculateValueRules<'a, C> + Debug,
{
    // ============================= LOG =============================
    debug!("the local rule stack `{}` is received", {
        rule_ref.get_str()
    });
    // ============================= LOG =============================
    let mut temp_stack: VecDeque<(&R::RuleType, CaptureData<C>)> = VecDeque::new();
    while let Some(mut frame) = stack.pop_front() {
        // ============================= LOG =============================
        debug!(
            "\ncheck the state of the rule `({}, {:#?})` \nfrom the local stack `{}`",
            frame.0.get_str(),
            frame.0.get_requirement(),
            rule_ref.get_str()
        );
        // ============================= LOG =============================

        match NextStep::next_or_finish_or_error(frame.0, &mut frame.1) {
            NextStep::Go => {
                let mut counter_of_each_rule: HashMap<usize, usize> = HashMap::new();
                // ============================= LOG =============================
                debug!(
                    "success, run subrules from the root rule `({}, {:#?})`",
                    rule_ref.get_str(),
                    rule_ref.get_requirement()
                );
                // ============================= LOG =============================

                for text in &frame.1.text_for_capture {
                    if let Some(simple_rules) = &frame.0.get_simple_rules() {
                        for index in R::get_selected_rules(simple_rules.1, text) {
                            let rule_from_regexset = simple_rules.0.get_index(index).unwrap();
                            // ============================= LOG =============================
                            trace!(
                                "found `({}, {:#?})` rule from `RegexSet` for `{:#?}` data",
                                rule_from_regexset.get_str(),
                                rule_from_regexset.get_requirement(),
                                text
                            );
                            // ============================= LOG =============================
                            let mut captures = R::find_captures(rule_from_regexset, text);
                            if let NextStep::Error(error) =
                                NextStep::next_or_finish_or_error(rule_from_regexset, &mut captures)
                            {
                                return NextStep::Error(error);
                            }
                            *counter_of_each_rule.entry(index).or_insert(0) += 1;
                            if counter_of_each_rule[&index] == frame.1.text_for_capture.len() {
                                // ============================= LOG =============================
                                trace!(
                                    "the ({}, {:#?}) rule worked successfully for all matches",
                                    rule_from_regexset.get_str(),
                                    rule_from_regexset.get_requirement(),
                                );
                                // ============================= LOG =============================
                                temp_stack.push_back((&rule_from_regexset, captures));
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
                // ============================= LOG =============================
            }
            NextStep::Error(_) => {}
        }
    }
    NextStep::Finish
}
