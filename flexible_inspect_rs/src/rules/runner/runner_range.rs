use super::*;
use range::RangeFormat;

impl RangeFormat {
    pub fn check_range(&self, capture: IndexSet<&[u8]>) -> bool {
        if let RangeFormat::Bytes(range) = self {
            
        }
        false
    }
}
