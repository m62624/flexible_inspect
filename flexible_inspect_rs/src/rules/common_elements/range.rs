use super::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Range {
    I32(std::ops::RangeInclusive<i32>),
    I64(std::ops::RangeInclusive<i64>),
    I128(std::ops::RangeInclusive<i128>),
    F32(std::ops::RangeInclusive<f32>),
    F64(std::ops::RangeInclusive<f64>),
}

enum RangeCheckMode {
    Any,
    All,
    Exactly(usize),
}

pub struct RangeChecker {
    mode: RangeCheckMode,
}

impl RangeChecker {
    fn new(mode: RangeCheckMode) -> Self {
        Self { mode }
    }

    pub fn any() -> Self {
        Self::new(RangeCheckMode::Any)
    }

    pub fn all() -> Self {
        Self::new(RangeCheckMode::All)
    }

    pub fn exactly(count: usize) -> Self {
        Self::new(RangeCheckMode::Exactly(count))
    }
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
