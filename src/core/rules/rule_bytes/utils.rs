use super::*;
use crate::core::rules::traits::CalculateValueRules;

impl CalculateValueRules<'_, &[u8]> for RuleBytes {
    type RegexSet = regex::bytes::RegexSet;
    type RuleType = RuleBytes;
    fn get_selected_rules(regex_set: &Self::RegexSet, text: &[u8]) -> Vec<usize> {
        regex_set.matches(text).into_iter().collect()
    }

    fn find_captures<'a>(rule: &Self::RuleType, capture: &'a [u8]) -> CaptureData<&'a [u8]> {
        captures::find_captures(rule, capture)
    }
}
