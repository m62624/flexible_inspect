use super::*;

pub fn number_range_status<R: RuleBase, C: PartialEq + Eq + Hash>(
    rule: &R,
    captures: &mut CaptureData<C>,
) -> NextStep {
    if let Some(rng) = rule.get_range() {
        match &rng {
            range::RangeFormat::Str(range_str) => match &range_str.range {
                range::RangeBoundaries::I8(_) => todo!(),
                range::RangeBoundaries::I16(_) => todo!(),
                range::RangeBoundaries::I32(_) => todo!(),
                range::RangeBoundaries::I64(_) => todo!(),
                range::RangeBoundaries::I128(_) => todo!(),
                range::RangeBoundaries::F32(_) => todo!(),
                range::RangeBoundaries::F64(_) => todo!(),
            },
            range::RangeFormat::Bytes(range_bytes) => match &range_bytes.range {
                range::RangeBoundaries::I8(_) => todo!(),
                range::RangeBoundaries::I16(_) => todo!(),
                range::RangeBoundaries::I32(_) => todo!(),
                range::RangeBoundaries::I64(_) => todo!(),
                range::RangeBoundaries::I128(_) => todo!(),
                range::RangeBoundaries::F32(_) => todo!(),
                range::RangeBoundaries::F64(_) => todo!(),
            },
        }
    }
    NextStep::Finish
}
