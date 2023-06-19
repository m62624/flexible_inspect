use super::*;
use std::hash::{Hash, Hasher};

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
        self.requirements.hash(state);
        self.rules_for_the_rule.hasher();
    }
}
