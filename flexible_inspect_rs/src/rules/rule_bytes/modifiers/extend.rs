use log::debug;

use super::*;
use crate::rules::traits::RuleBase;

/// a method for extending the rule with nested rules
pub fn extend<R: IntoIterator<Item = RuleBytes>>(
    rule: &mut RuleBytes,
    nested_rules: R,
) -> RuleBytes {
    let subrules: IndexSet<_> = nested_rules.into_iter().collect();
    rule.content_mut_unchecked().subrules_bytes = if !subrules.is_empty() {
        Some(SimpleRulesBytes::new(subrules))
    } else {
        None
    };
    debug!(
        "the `extend` modifier is applied to rule ({}, {:#?})",
        rule.get_str(),
        rule.get_requirement()
    );
    std::mem::take(rule)
}
