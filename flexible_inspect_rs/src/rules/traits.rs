/*
Here we implement traits for two types of `Rule`, they are string `Rule` and byte `Rule`.
They are necessary to avoid code duplicates. Especially in context_match, where there are several modes
*/

// =======================================================
use super::{CaptureData, Counter, ModeMatch};
use crate::prelude::MatchRequirement;
use indexmap::IndexSet;
use std::{fmt::Debug, hash::Hash};
// =======================================================

/// This trait requires implementations of the most basic methods for any `Rule`.
pub trait RuleBase {
    type TakeRuleType;
    type SubRulesType;
    type RuleType: Debug;
    type RegexSet;

    fn _new<T: Into<String>>(pattern: T, requirement: MatchRequirement) -> Self;
    fn content_unchecked(&self) -> &Self::TakeRuleType;
    fn content_mut_unchecked(&mut self) -> &mut Self::TakeRuleType;
    fn get_subrules(&self) -> Option<&Self::SubRulesType>;
    fn get_simple_rules(&self) -> Option<(&IndexSet<Self::RuleType>, &Self::RegexSet)>;
    fn get_complex_rules(&self) -> Option<&IndexSet<Self::RuleType>>;
    fn get_requirement(&self) -> &MatchRequirement;
    fn get_counter(&self) -> Option<Counter>;
    fn get_mode_match(&self) -> &ModeMatch;
    fn get_str(&self) -> &str;
}

/// The main trait for `context_match`, that is,
/// the implementation of the modifier nesting logic will be common for two different rule structures.
/// That is, `next` + `mode matching` will be common for them.
/// The main thing is to implement separately `Captures` for `&str` and `&[u8]`
/// the rest will be the same

pub trait CalculateValueRules<'a, C: PartialEq + Eq + Hash> {
    type RuleType: RuleBase<RuleType = Self::RuleType, RegexSet = Self::RegexSet>
        + Hash
        + Eq
        + PartialEq;
    type RegexSet: 'a;
    fn get_selected_rules(regex_set: &Self::RegexSet, text: &C) -> Vec<usize>;
    fn find_captures(rule: &Self::RuleType, capture: &C) -> CaptureData<C>;
}

/// This trait requires modifier implementations for any `Rules`
pub trait RuleModifiers {
    type RuleType;

    /// modifier for extending the rule with nested rules
    /// ( **by default, all rules must pass every match check** )
    fn extend<R: IntoIterator<Item = Self::RuleType>>(&mut self, nested_rules: R)
        -> Self::RuleType;
    /// modifier to set the match counter, condition counter == match
    fn counter_is_equal(&mut self, count: usize) -> Self::RuleType;
    /// modifier to set the match counter, condition counter >= match
    fn counter_more_than(&mut self, count: usize) -> Self::RuleType;
    /// modifier to set the match counter, condition counter <= match
    fn counter_less_than(&mut self, count: usize) -> Self::RuleType;
    /// modifier to change the rule matching mode,
    /// `all rules` must pass the test for at least `one match`
    fn mode_all_rules_for_at_least_one_match(&mut self) -> Self::RuleType;
    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for `all matches`
    fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self::RuleType;
    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for at least `one match`
    fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::RuleType;
}
