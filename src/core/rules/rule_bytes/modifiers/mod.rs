mod extend;
use super::*;
use crate::core::rules::traits::RuleModifiers;

impl RuleModifiers for RuleBytes {
    type RuleType = RuleBytes;
    /// a method for extending the rule with nested rules
    fn extend<T: IntoIterator<Item = Self::RuleType>>(
        &mut self,
        nested_rules: T,
    ) -> Self::RuleType {
        extend::extend(self, nested_rules)
    }
}
