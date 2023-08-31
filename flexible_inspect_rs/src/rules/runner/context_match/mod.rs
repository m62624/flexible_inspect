// =======================================================
mod all_rules_for_all_matches;
mod all_rules_for_at_least_one_match;
mod at_least_one_rule_for_all_matches;
mod at_least_one_rule_for_at_least_one_match;
// =======================================================
use super::*;
use crate::rules::traits::RuleBase;
use crate::rules::{next::NextStep, traits::CalculateValueRules, CaptureData};
pub use all_rules_for_all_matches::all_rules_for_all_matches;
pub use all_rules_for_at_least_one_match::all_rules_for_at_least_one_match;
pub use at_least_one_rule_for_all_matches::at_least_one_rule_for_all_matches;
pub use at_least_one_rule_for_at_least_one_match::at_least_one_rule_for_at_least_one_match;
use log::{debug, error, info, trace};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

#[derive(Debug)]
pub struct FrameStack<'a, R, C>
where
    R: CalculateValueRules<'a, C>,
    C: IntoSpecificCaptureType<'a>,
{
    rule: &'a R::RuleType,
    capture: CaptureData<'a, C>,
}

impl<'a, R, C> FrameStack<'a, R, C>
where
    R: CalculateValueRules<'a, C>,
    C: IntoSpecificCaptureType<'a>,
{
    pub fn new(rule: &'a R::RuleType, capture: CaptureData<'a, C>) -> Self {
        Self { rule, capture }
    }
}

// pub fn rules_in_category_regexset<'a, R, C, F>(
//     // get a unique stack of one root rule, necessary to bypass the recursion constraint
//     frame: FrameStack<'a, R, C>,
//     callback: F,
// ) -> NextStep
// where
//     R: CalculateValueRules<'a, C>,
//     C: IntoSpecificCaptureType<'a>,
//     F: FnOnce(FrameStack<'a, R, C>) -> NextStep,
// {
//     if let Some(simple_rules) = frame.rule.get_simple_rules() {
//         return callback(frame);
//     }
//     NextStep::Finish
// }
