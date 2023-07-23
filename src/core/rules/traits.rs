use crate::{Rule, RuleBytes};

use super::{rule_bytes::CaptureDataBytes, rule_str::CaptureDataStr};

/// This trait requires implementations of the most basic methods for any `Rule`.
pub trait RuleBase {
    type TakeRuleType;
    fn content_unchecked(&self) -> &Self::TakeRuleType;
    fn content_mut_unchecked(&mut self) -> &mut Self::TakeRuleType;
}

/// This trait requires method implementations that are different for different structures
pub trait RuleExtendBase<'s>: RuleBase {
    fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize>;
    fn find_captures(rule: &Rule, text: &'s str) -> CaptureDataStr<'s>;
}

/// This trait requires method implementations that are different for different structures
pub trait RuleBytesExtendBase<'s>: RuleBase {
    fn get_selected_rules(regex_set: &regex::bytes::RegexSet, text_bytes: &[u8]) -> Vec<usize>;
    fn find_captures(rule: &RuleBytes, text_bytes: &'s [u8]) -> CaptureDataBytes<'s>;
}

/// This trait requires modifier implementations for any Rules
pub trait RuleModifiers {
    type RuleType;

    fn extend<T: IntoIterator<Item = Self::RuleType>>(&mut self, nested_rules: T)
        -> Self::RuleType;
}
