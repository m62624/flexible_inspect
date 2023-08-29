use super::range::RangeFormat;
use super::rules::traits::{CalculateValueRules, RangeType};
use super::*;

impl<'a> CalculateValueRules<'a, &'a [u8]> for RuleBytes {
    type RegexSet = regex::bytes::RegexSet;
    type RuleType = RuleBytes;
    fn get_selected_rules(regex_set: &Self::RegexSet, text: &&[u8]) -> Vec<usize> {
        regex_set.matches(text).into_iter().collect()
    }

    fn find_captures(rule: &Self::RuleType, capture: &&'a [u8]) -> CaptureData<'a, &'a [u8]> {
        captures::find_captures(rule, capture)
    }
}

impl RuleBytes {
    /// Before using, create a rule with a regular expression that accepts a number, example of regular expressions for integers and fractions.
    /// - `\d+`, `\b\d+\b`.
    /// - `[-+]?[0-9]*\.?[0-9]+(?:[eE][-+]?[0-9]+)?`\
    /// after, the method automatically converts the received data into a number (all numbers can be of different ranges and data types).
    /// - Supported data types : `i8`, `i16`, `i32`, `i64`, `i128`, `f32`, `f64`
    ///
    /// # Notes
    /// Each signed variant can store numbers from `-(2^n - 1) to 2^(n - 1) - 1` inclusive, where n is the number of bits that variant uses. So an `i32` can store numbers from `-(2^31)` to `2^31 - 1`, which equals `-2147483648` to `2147483647`.
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
