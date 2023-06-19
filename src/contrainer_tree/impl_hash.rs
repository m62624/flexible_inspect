use super::*;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

impl Hash for ContainerTree {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rules.hash(state);
    }
}

