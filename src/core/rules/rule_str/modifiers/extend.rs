use super::*;
use crate::core::rules::traits::RuleBase;

/// a method for extending the rule with nested rules
pub fn extend<R: IntoIterator<Item = Rule>>(rule: &mut Rule, nested_rules: R) -> Rule {
    let sliced_rules = SlisedRules::new(nested_rules);
    if sliced_rules.is_some() {
        rule.content_mut_unchecked().subrules = Some(Subrules::new(
            SimpleRules::new(sliced_rules.simple_rules),
            sliced_rules.complex_rules,
        ));
    }
    std::mem::take(rule)
}
