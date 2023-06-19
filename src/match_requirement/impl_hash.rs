use super::*;
use std::hash::{Hash, Hasher};

impl Hash for MatchRequirement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
