use super::*;
use crate::core::rules::traits::CalculateValueRules;

impl CalculateValueRules <&[u8]> for Rule {
    type RegexSet = regex::bytes::RegexSet;
    type RuleType = RuleBytes;
    fn get_selected_rules(regex_set: Self::RegexSet, text: &[u8]) -> Vec<usize> {
        regex_set.matches(text).into_iter().collect()
    }

    fn find_captures(rule: Self::RuleType, capture: &[u8]) -> CaptureData<&[u8]> {
        captures::find_captures(rule, capture)
    }
}
