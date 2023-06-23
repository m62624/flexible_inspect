use super::*;
use std::cmp::{Eq, PartialEq};
impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.rule_raw == other.rule_raw
            && self.requirement == other.requirement
            && self.rules_for_the_rule == other.rules_for_the_rule
    }
}
impl Eq for Rule {}
