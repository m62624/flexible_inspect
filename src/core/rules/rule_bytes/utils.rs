use super::*;
use crate::core::rules::{captures::CaptureData, traits::RuleBytesExtendBase};

impl<'a> RuleBytesExtendBase<'a> for RuleBytes {
    type RuleType = RuleBytes;
    /// Get selected rules from `RegexSet`
    fn get_selected_rules(regex_set: &regex::bytes::RegexSet, text_bytes: &[u8]) -> Vec<usize> {
        regex_set.matches(text_bytes).iter().collect()
    }

    fn find_captures(rule: &Self::RuleType, text_bytes: &'a [u8]) -> CaptureData<'a> {
        captures::find_captures(rule, text_bytes)
    }

    fn run(rule: &Self::RuleType, text: &[u8]) -> next::NextStep {
        todo!()
    }
}
