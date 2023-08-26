/*
Here we implement traits for two types of `Rule`, they are string `Rule` and byte `Rule`.
They are necessary to avoid code duplicates. Especially in context_match, where there are several modes
*/

// =======================================================
use super::{CaptureData, Counter, ModeMatch, Range};
use crate::prelude::MatchRequirement;
use indexmap::IndexSet;
use std::hash::Hasher;
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

    fn number_range<T: PartialOrd>(self, range: std::ops::RangeInclusive<T>) -> Self::RuleType;
}

mod for_Range {
    use super::*;

    impl Hash for Range {
        fn hash<H: Hasher>(&self, state: &mut H) {
            match self {
                Range::I32(range) => range.hash(state),
                Range::I64(range) => range.hash(state),
                Range::I128(range) => range.hash(state),
                Range::F32(range) => {
                    range.start().to_bits().hash(state);
                    range.end().to_bits().hash(state);
                }
                Range::F64(range) => {
                    range.start().to_bits().hash(state);
                    range.end().to_bits().hash(state);
                }
            }
        }
    }

    impl PartialEq for Range {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Range::I32(range1), Range::I32(range2)) => range1 == range2,
                (Range::I64(range1), Range::I64(range2)) => range1 == range2,
                (Range::I128(range1), Range::I128(range2)) => range1 == range2,
                (Range::F32(range1), Range::F32(range2)) => {
                    range1.start().to_bits() == range2.start().to_bits()
                        && range1.end().to_bits() == range2.end().to_bits()
                }
                (Range::F64(range1), Range::F64(range2)) => {
                    range1.start().to_bits() == range2.start().to_bits()
                        && range1.end().to_bits() == range2.end().to_bits()
                }
                _ => false,
            }
        }
    }

    impl Eq for Range {}
}
