use super::range::RangeFormat;
use super::rules::traits::{CalculateValueRules, RangeType};
use super::*;

impl CalculateValueRules<'_, &[u8]> for RuleBytes {
    type RegexSet = regex::bytes::RegexSet;
    type RuleType = RuleBytes;
    fn get_selected_rules(regex_set: &Self::RegexSet, text: &&[u8]) -> Vec<usize> {
        regex_set.matches(text).into_iter().collect()
    }

    fn find_captures<'a>(rule: &Self::RuleType, capture: &&'a [u8]) -> CaptureData<&'a [u8]> {
        captures::find_captures(rule, capture)
    }
}

impl RuleBytes {
    pub fn number_range<RNG: RangeType>(
        mut self,
        range: RNG,
        read_bytes_mode: ReadMode,
        range_mode: RangeMode,
    ) -> RuleBytes {
        self.0.general_modifiers.range = Some(RangeFormat::Bytes(RangeBytes::new(
            range.get_range(),
            read_bytes_mode,
            range_mode,
        )));
        self
    }
}
