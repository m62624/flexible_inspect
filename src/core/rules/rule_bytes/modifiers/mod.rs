mod extend;
use super::*;
use crate::core::rules::traits::{RuleBase, RuleModifiers};

impl RuleModifiers for RuleBytes {
    type RuleType = RuleBytes;
    /// a method for extending the rule with nested rules
    fn extend<T: IntoIterator<Item = Self::RuleType>>(
        &mut self,
        nested_rules: T,
    ) -> Self::RuleType {
        extend::extend(self, nested_rules)
    }

    fn counter_is_equal(&mut self, count: usize) -> Self::RuleType {
        self.content_mut_unchecked().general_modifiers.counter = Some(Counter::Only(count));
        std::mem::take(self)
    }

    fn counter_more_than(&mut self, count: usize) -> Self::RuleType {
        self.content_mut_unchecked().general_modifiers.counter = Some(Counter::MoreThan(count));
        std::mem::take(self)
    }

    fn counter_less_than(&mut self, count: usize) -> Self::RuleType {
        self.content_mut_unchecked().general_modifiers.counter = Some(Counter::LessThan(count));
        std::mem::take(self)
    }
}
