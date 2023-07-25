/*
Here we implement traits for two types of `Rule`, they are string `Rule` and byte `Rule`.
They are necessary to avoid code duplicates. Especially in context_match, where there are several modes
*/

use super::{CaptureData, Counter, ModeMatch};
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
    fn get_mode_match(&self) -> &ModeMatch;
    fn as_str(&self) -> &str;
}

/// The main trait for `context_match`, that is,
/// the implementation of the modifier nesting logic will be common for two different rule structures.
/// That is, `next` + `mode matching` will be common for them.
/// The main thing is to implement separately `Captures` for `&str` and `&[u8]`
/// the rest will be the same
pub trait CalculateValueRules<T: PartialEq + Eq + Hash> {
    type RuleType;
    type RegexSet;
    fn get_selected_rules(regex_set: Self::RegexSet, text: T) -> Vec<usize>;
    fn find_captures<'a>(rule: &Self::RuleType, capture: T) -> CaptureData<T>;
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
