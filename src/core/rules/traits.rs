use super::{CaptureData, Counter};
use crate::MatchRequirement;
use std::hash::Hash;

/// This trait requires implementations of the most basic methods for any `Rule`.
pub trait RuleBase {
    type TakeRuleType;
    type SubRulesType;
    fn content_unchecked(&self) -> &Self::TakeRuleType;
    fn content_mut_unchecked(&mut self) -> &mut Self::TakeRuleType;
    fn get_subrules(&self) -> Option<&Self::SubRulesType>;
    fn get_requirement(&self) -> &MatchRequirement;
    fn get_counter(&self) -> Option<Counter>;
}

pub trait CalculateValueRules<RGX, RGSST, INPT: PartialEq + Eq + Hash> {
    fn get_selected_rules(regex_set: RGSST, text: INPT) -> Vec<usize>;
    fn find_captures(rule: RGX, capture: INPT) -> CaptureData<INPT>;
}

/// This trait requires modifier implementations for any `Rules`
pub trait RuleModifiers {
    type RuleType;

    /// a method for extending the rule with nested rules
    fn extend<T: IntoIterator<Item = Self::RuleType>>(&mut self, nested_rules: T)
        -> Self::RuleType;

    fn counter_is_equal(&mut self, count: usize) -> Self::RuleType;
    fn counter_more_than(&mut self, count: usize) -> Self::RuleType;
    fn counter_less_than(&mut self, count: usize) -> Self::RuleType;

    fn mode_all_rules_for_at_least_one_match(&mut self) -> Self::RuleType;
    fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self::RuleType;
    fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::RuleType;
}
