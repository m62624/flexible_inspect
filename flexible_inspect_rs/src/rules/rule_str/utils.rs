use super::range::RangeFormat;
use super::rule_str::Rule;
use super::*;
use crate::rules::traits::{CalculateValueRules, RangeType};

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

impl Rule {
    pub fn number_range<RNG: RangeType>(mut self, range: RNG, range_mode: RangeMode) -> Rule {
        self.0.general_modifiers.range = Some(RangeFormat::Str(RangeStr::new(
            range.get_range(),
            range_mode,
        )));
        self
    }
}
