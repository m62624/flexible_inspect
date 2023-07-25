use super::*;
use crate::core::rules::traits::CalculateValueRules;

impl CalculateValueRules<RuleBytes, regex::bytes::RegexSet, &[u8]> for Rule {
    fn get_selected_rules(regex_set: regex::bytes::RegexSet, text: &[u8]) -> Vec<usize> {
        regex_set.matches(text).into_iter().collect()
    }

    fn find_captures(rule: RuleBytes, capture: &[u8]) -> CaptureData<&[u8]> {
        captures::find_captures(rule, capture)
    }
}
