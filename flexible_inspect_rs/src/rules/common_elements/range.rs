use super::traits::RangeType;
use super::*;
use std::ops::RangeInclusive;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Range {
    pub range: RangeBoundaries,
    pub mode: RangeMode,
}

impl Range {
    pub fn new<T: RangeType>(range: T, mode: RangeMode) -> Self {
        Self {
            range: range.get_range(),
            mode,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum RangeBoundaries {
    I32(RangeInclusive<i32>),
    I64(RangeInclusive<i64>),
    I128(RangeInclusive<i128>),
    F32(RangeInclusive<f32>),
    F64(RangeInclusive<f64>),
}

impl RangeType for RangeInclusive<i32> {
    fn get_range(self) -> RangeBoundaries {
        RangeBoundaries::I32(self)
    }
}

impl RangeType for RangeInclusive<i64> {
    fn get_range(self) -> RangeBoundaries {
        RangeBoundaries::I64(self)
    }
}

impl RangeType for RangeInclusive<i128> {
    fn get_range(self) -> RangeBoundaries {
        RangeBoundaries::I128(self)
    }
}

impl RangeType for RangeInclusive<f32> {
    fn get_range(self) -> RangeBoundaries {
        RangeBoundaries::F32(self)
    }
}

impl RangeType for RangeInclusive<f64> {
    fn get_range(self) -> RangeBoundaries {
        RangeBoundaries::F64(self)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RangeMode {
    Any,
    All,
    Exactly(usize),
}

impl Hash for RangeBoundaries {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RangeBoundaries::I32(range) => range.hash(state),
            RangeBoundaries::I64(range) => range.hash(state),
            RangeBoundaries::I128(range) => range.hash(state),
            RangeBoundaries::F32(range) => {
                range.start().to_bits().hash(state);
                range.end().to_bits().hash(state);
            }
            RangeBoundaries::F64(range) => {
                range.start().to_bits().hash(state);
                range.end().to_bits().hash(state);
            }
        }
    }
}

impl PartialEq for RangeBoundaries {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RangeBoundaries::I32(range1), RangeBoundaries::I32(range2)) => range1 == range2,
            (RangeBoundaries::I64(range1), RangeBoundaries::I64(range2)) => range1 == range2,
            (RangeBoundaries::I128(range1), RangeBoundaries::I128(range2)) => range1 == range2,
            (RangeBoundaries::F32(range1), RangeBoundaries::F32(range2)) => {
                range1.start().to_bits() == range2.start().to_bits()
                    && range1.end().to_bits() == range2.end().to_bits()
            }
            (RangeBoundaries::F64(range1), RangeBoundaries::F64(range2)) => {
                range1.start().to_bits() == range2.start().to_bits()
                    && range1.end().to_bits() == range2.end().to_bits()
            }
            _ => false,
        }
    }
}

impl Eq for RangeBoundaries {}
