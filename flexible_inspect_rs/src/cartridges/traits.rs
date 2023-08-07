use super::*;

/// This trait is required for single access to `Rule cartridges` or `RuleBytes cartridges`
pub trait CartridgeBase<D>
where
    D: PartialEq + Eq + Hash + Debug,
{
    /// Run the validation for one `cartridge`
    fn run(&self, data: D) -> NextStep;
    /// Get the `error code`
    fn get_id(&self) -> i32;
    /// Get an `error message` with data
    fn get_message(&self) -> &str;
}

pub trait CartridgeModifiers {
    type CartridgeType;
    /// modifier to change the rule matching mode,
    /// `all rules` must pass the test for at least `one match`
    fn mode_all_rules_for_at_least_one_match(&mut self) -> Self::CartridgeType;

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for `all matches`
    fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self::CartridgeType;

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for at least `one match`
    fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::CartridgeType;
}
