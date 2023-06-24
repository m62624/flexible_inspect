use super::*;
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.rule_raw.hash(state);
        self.rules_for_the_rule.hash(state);
    }
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.rule_raw == other.rule_raw && self.rules_for_the_rule == other.rules_for_the_rule
    }
}
impl Eq for Rule {}
