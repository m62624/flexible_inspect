use super::captures::CaptureData;
use crate::{MatchRequirement, Rule, RuleBytes};

/// This trait requires implementations of the most basic methods for any `Rule`.
pub trait RuleBase {
    type TakeRuleType;
    type SubRulesType;
    fn content_unchecked(&self) -> &Self::TakeRuleType;
    fn content_mut_unchecked(&mut self) -> &mut Self::TakeRuleType;
    fn get_subrules(&self) -> Option<&Self::SubRulesType>;
    fn get_requirement(&self) -> &MatchRequirement;
}

/// This trait requires method implementations that are different for different structures
pub trait RuleExtendBase<'s>: RuleBase {
    /// This method returns the indices of the selected rules that will exactly trigger based on the data ( `&str` | `&[u8]` )
    fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize>;

    /// This method finds matches on a regular expression.
    /// Saves one first match for each group to fill in the error message, and all matches to pass on to the next rule check
    fn find_captures(rule: &Rule, text: &'s str) -> CaptureData<'s>;
}

/// This trait requires method implementations that are different for different structures
pub trait RuleBytesExtendBase<'s>: RuleBase {
    /// This method returns the indices of the selected rules that will exactly trigger based on the data ( `&str` | `&[u8]` )
    fn get_selected_rules(regex_set: &regex::bytes::RegexSet, text_bytes: &[u8]) -> Vec<usize>;

    /// This method finds matches on a regular expression.
    /// Saves one first match for each group to fill in the error message, and all matches to pass on to the next rule check
    fn find_captures(rule: &RuleBytes, text_bytes: &'s [u8]) -> CaptureData<'s>;
}

/// This trait requires modifier implementations for any `Rules`
pub trait RuleModifiers {
    type RuleType;

    /// a method for extending the rule with nested rules
    fn extend<T: IntoIterator<Item = Self::RuleType>>(&mut self, nested_rules: T)
        -> Self::RuleType;
}
