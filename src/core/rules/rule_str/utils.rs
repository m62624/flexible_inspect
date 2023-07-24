use crate::core::rules::{captures::CaptureData, traits::RuleExtendBase};

use super::*;

impl<'a> RuleExtendBase<'a> for Rule {
    type RuleType = Rule;
    /// Get selected rules from `RegexSet`
    fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize> {
        regex_set.matches(text).iter().collect()
    }

    fn find_captures(rule: &Self::RuleType, text: &'a str) -> CaptureData<'a> {
        captures::find_captures(rule, text)
    }

    fn run(rule: &Self::RuleType, text: &str) -> next::NextStep {
        todo!()
    }
}
