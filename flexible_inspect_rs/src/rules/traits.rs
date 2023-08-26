/*
Here we implement traits for two types of `Rule`, they are string `Rule` and byte `Rule`.
They are necessary to avoid code duplicates. Especially in context_match, where there are several modes
*/

// =======================================================
use super::common_elements::range::RangeChecker;
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
    /// The type of the rule that will be returned after applying the modifier
    type RuleType;

    /// modifier for extending the rule with nested rules
    ///
    /// ( **by default, `all_rules_for_all_matches`** )\
    /// In this mode, all rules must be tested for all matches
    fn extend<R: IntoIterator<Item = Self::RuleType>>(self, nested_rules: R) -> Self::RuleType;
    /// modifier to set the match counter, condition `counter == match`
    fn counter_is_equal(self, count: usize) -> Self::RuleType;
    /// modifier to set the match counter, condition `counter >= match`
    fn counter_more_than(self, count: usize) -> Self::RuleType;
    /// modifier to set the match counter, condition `counter <= match`
    fn counter_less_than(self, count: usize) -> Self::RuleType;
    /// modifier to change the rule matching mode.
    ///
    /// In this mode, all rules must pass the test for at least one match
    fn all_r_for_any_m(self) -> Self::RuleType;
    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least one rule must pass the test for all matches.
    fn any_r_for_all_m(self) -> Self::RuleType;
    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least one rule must pass at least one match check
    fn any_r_for_any_m(self) -> Self::RuleType;

    /// Before using, create a rule with a regular expression that accepts a number, example of regular expressions for integers and fractions.
    /// - `\d+`, `\b\d+\b`.
    /// - `[-+]?[0-9]*\.?[0-9]+(?:[eE][-+]?[0-9]+)?`\
    /// after, the method automatically converts the received data into a number (all numbers can be of different ranges and data types),
    /// Supported data types : `i32`, `i64`, `i128`, `f32`, `f64`
    ///
    /// # Notes
    /// Each signed variant can store numbers from `-(2^n - 1) to 2^(n - 1) - 1` inclusive, where n is the number of bits that variant uses. So an `i32` can store numbers from `-(2^31)` to `2^31 - 1`, which equals `-2147483648` to `2147483647`.
    fn number_range<T: PartialOrd>(
        self,
        range: std::ops::RangeInclusive<T>,
        mode: RangeChecker,
    ) -> Self::RuleType;
}
