use super::*;
impl PartialEq for ContainerTree {
    fn ne(&self, other: &Self) -> bool {
        self.rules != other.rules
    }

    fn eq(&self, other: &Self) -> bool {
        self.rules == other.rules
    }
}

impl Eq for ContainerTree {}
