use super::*;
use crate::core::rules::traits::CalculateValueRules;

impl CalculateValueRules<'_, &str> for Rule {
    type RegexSet = regex::RegexSet;
    type RuleType = Rule;
    fn get_selected_rules(regex_set: &Self::RegexSet, text: &&str) -> Vec<usize> {
        regex_set.matches(text).into_iter().collect()
    }

    fn find_captures<'a>(rule: &Self::RuleType, capture: &&'a str) -> rules::CaptureData<&'a str> {
        captures::find_captures(rule, capture)
    }
}
