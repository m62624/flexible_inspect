use super::*;
use crate::core::rules::traits::RuleBase;

pub fn extend<T: IntoIterator<Item = Rule>>(rule: &mut Rule, nested_rules: T) -> Rule {
    let sliced_rules = SlisedRules::new(nested_rules);
    if sliced_rules.is_some() {
        rule.content_mut_unchecked().subrules = Some(Subrules::new(
            SimpleRules::new(sliced_rules.simple_rules),
            sliced_rules.complex_rules,
        ));
    }
    std::mem::take(rule)
}
