use super::*;
use crate::rules::traits::RuleBase;
use log::debug;

/// a method for extending the rule with nested rules
pub fn extend<R: IntoIterator<Item = Rule>>(rule: &mut Rule, nested_rules: R) -> Rule {
    let sliced_rules = SlisedRules::new(nested_rules);
    if sliced_rules.is_some() {
        rule.content_mut_unchecked().subrules = Some(Subrules::new(
            SimpleRules::new(sliced_rules.simple_rules),
            sliced_rules.complex_rules,
        ));
    }
    debug!(
        "the `extend` modifier is applied to rule ({}, {:#?})",
        rule.get_str(),
        rule.get_requirement()
    );
    std::mem::take(rule)
}
