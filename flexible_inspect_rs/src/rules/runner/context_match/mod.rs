// =======================================================
mod all_rules_for_all_matches;
mod all_rules_for_at_least_one_match;
mod at_least_one_rule_for_all_matches;
mod at_least_one_rule_for_at_least_one_match;
// =======================================================
use super::*;
use crate::rules::{next::NextStep, traits::CalculateValueRules, CaptureData};
pub use all_rules_for_all_matches::all_rules_for_all_matches;
pub use all_rules_for_at_least_one_match::all_rules_for_at_least_one_match;
pub use at_least_one_rule_for_all_matches::at_least_one_rule_for_all_matches;
pub use at_least_one_rule_for_at_least_one_match::at_least_one_rule_for_at_least_one_match;
use log::{debug, error, info, trace};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

pub type Stack<'a, R: CalculateValueRules<'a, C>, C: IntoSpecificCaptureType<'a>> =
    VecDeque<(R::RuleType, CaptureData<'a, C>)>;

pub fn part_one<'a, R, C, F>(func: F, temp_stack: Option<Stack<'a, R, C>>) -> NextStep
where
    R: CalculateValueRules<'a, C>,
    C: IntoSpecificCaptureType<'a> + 'a,
    F: FnMut(Stack<'a, R, C>) -> NextStep,
{
    NextStep::Finish
}

pub fn part_two<'a, R, C, F>(func: F, temp_stack: Option<Stack<'a, R, C>>) -> NextStep
where
    R: CalculateValueRules<'a, C>,
    C: IntoSpecificCaptureType<'a> + 'a,
    F: FnMut(Stack<'a, R, C>) -> NextStep,
{
    NextStep::Finish
}

pub fn part_three<'a, R, C, F>(func: F, temp_stack: Option<Stack<'a, R, C>>) -> NextStep
where
    R: CalculateValueRules<'a, C>,
    C: IntoSpecificCaptureType<'a> + 'a,
    F: FnMut(Stack<'a, R, C>) -> NextStep,
{
    NextStep::Finish
}
