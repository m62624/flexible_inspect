use super::*;
use crate::core::rules::traits::CalculateValueRules;

impl CalculateValueRules<Rule, &regex::RegexSet, &str> for Rule {
    fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize> {
        regex_set.matches(text).into_iter().collect()
    }

    fn find_captures(rule: Rule, capture: &str) -> CaptureData<&str> {
        captures::find_captures(rule, capture)
    }
}
