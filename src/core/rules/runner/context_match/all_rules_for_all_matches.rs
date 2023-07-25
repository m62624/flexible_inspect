use crate::core::rules::next::NextStep;
use crate::core::rules::traits::{CalculateValueRules, RuleBase};
use crate::core::rules::CaptureData;
use std::{collections::VecDeque, fmt::Debug, hash::Hash};

pub fn all_rules_for_all_matches<R, C>(
    stack: &mut VecDeque<(&R::RuleType, CaptureData<C>)>,
) -> NextStep
where
    C: PartialEq + Eq + Hash + Debug,
    R: CalculateValueRules<C>,
    R::RuleType: RuleBase,
{
    NextStep::Finish
}
