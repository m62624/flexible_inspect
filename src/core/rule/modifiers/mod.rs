mod bytes_rule;
use super::*;

impl Rule {
    /// The method for adding nested rules
    pub fn extend<T: IntoIterator<Item = Rule>>(&mut self, nested_rules: T) -> Self {
        let sliced_rules =
            SlisedRules::slice_rules(&self, nested_rules);
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
