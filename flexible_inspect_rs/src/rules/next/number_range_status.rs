use super::*;
use crate::rules::rule_bytes::runner_range::single_bytes_result;
use crate::rules::rule_str::runner_range::single_str_result;

pub fn number_range_status<'a, R: RuleBase, C: IntoSpecificCaptureType<'a>>(
    rule: &R,
    captures: &mut CaptureData<'a, C>,
) -> NextStep {
    if let Some(rng) = rule.get_range() {
        return match &rng {
            range::RangeFormat::Str(range_str) => match &range_str.range {
                range::RangeBoundaries::I8(value) => single_str_result(range_str, captures, value),
                range::RangeBoundaries::I16(value) => single_str_result(range_str, captures, value),
                range::RangeBoundaries::I32(value) => single_str_result(range_str, captures, value),
                range::RangeBoundaries::I64(value) => single_str_result(range_str, captures, value),
                range::RangeBoundaries::I128(value) => {
                    single_str_result(range_str, captures, value)
                }
                range::RangeBoundaries::F32(value) => single_str_result(range_str, captures, value),
                range::RangeBoundaries::F64(value) => single_str_result(range_str, captures, value),
            },
            range::RangeFormat::Bytes(range_bytes) => match &range_bytes.range {
                range::RangeBoundaries::I8(value) => {
                    single_bytes_result(range_bytes, captures, value)
                }
                range::RangeBoundaries::I16(value) => {
                    single_bytes_result(range_bytes, captures, value)
                }
                range::RangeBoundaries::I32(value) => {
                    single_bytes_result(range_bytes, captures, value)
                }
                range::RangeBoundaries::I64(value) => {
                    single_bytes_result(range_bytes, captures, value)
                }
                range::RangeBoundaries::I128(value) => {
                    single_bytes_result(range_bytes, captures, value)
                }
                range::RangeBoundaries::F32(value) => {
                    single_bytes_result(range_bytes, captures, value)
                }
                range::RangeBoundaries::F64(value) => {
                    single_bytes_result(range_bytes, captures, value)
                }
            },
        };
    }
    NextStep::Finish
}
