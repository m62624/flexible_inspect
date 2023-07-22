use super::*;
use crate::core::rule::slice::SlisedRules;

impl Rule {
    pub fn extend(&mut self, nested_rules: IndexSet<Rule>) -> Self {
        let sliced_rules = SlisedRules::slice_rules(nested_rules);
        if sliced_rules.is_some() {
            self.content_mut_unchecked().mutable_modifiers.subrules = Some(Subrules::new(
                SimpleRules::new(sliced_rules.simple_rules),
                sliced_rules.complex_rules,
                sliced_rules.bytes_rules,
            ));
        }
        std::mem::take(self)
    }
}
