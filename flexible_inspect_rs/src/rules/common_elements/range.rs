use super::traits::RangeType;
use super::*;
use std::ops::RangeInclusive;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Range {
    I32(RangeInclusive<i32>),
    I64(RangeInclusive<i64>),
    I128(RangeInclusive<i128>),
    F32(RangeInclusive<f32>),
    F64(RangeInclusive<f64>),
}

impl RangeType<i32> for RangeInclusive<i32> {
    fn get_range(self) -> Range {
        Range::I32(self)
    }
}

impl RangeType<i64> for RangeInclusive<i64> {
    fn get_range(self) -> Range {
        Range::I64(self)
    }
}

impl RangeType<i128> for RangeInclusive<i128> {
    fn get_range(self) -> Range {
        Range::I128(self)
    }
}

impl RangeType<f32> for RangeInclusive<f32> {
    fn get_range(self) -> Range {
        Range::F32(self)
    }
}

impl RangeType<f64> for RangeInclusive<f64> {
    fn get_range(self) -> Range {
        Range::F64(self)
    }
}

// fn set_range<T>(range: RangeInclusive<T>) -> Range
// where
//     T: PartialOrd + Copy,
//     RangeInclusive<T>: RangeType<T>,
// {
//     range.get_range()
// }

#[derive(Debug)]
pub enum RangeMode {
    Any,
    All,
    Exactly(usize),
}

impl Hash for Range {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Range::I32(range) => range.hash(state),
            Range::I64(range) => range.hash(state),
            Range::I128(range) => range.hash(state),
            Range::F32(range) => {
                range.start().to_bits().hash(state);
                range.end().to_bits().hash(state);
            }
            Range::F64(range) => {
                range.start().to_bits().hash(state);
                range.end().to_bits().hash(state);
            }
        }
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Range::I32(range1), Range::I32(range2)) => range1 == range2,
            (Range::I64(range1), Range::I64(range2)) => range1 == range2,
            (Range::I128(range1), Range::I128(range2)) => range1 == range2,
            (Range::F32(range1), Range::F32(range2)) => {
                range1.start().to_bits() == range2.start().to_bits()
                    && range1.end().to_bits() == range2.end().to_bits()
            }
            (Range::F64(range1), Range::F64(range2)) => {
                range1.start().to_bits() == range2.start().to_bits()
                    && range1.end().to_bits() == range2.end().to_bits()
            }
            _ => false,
        }
    }
}

impl Eq for Range {}
