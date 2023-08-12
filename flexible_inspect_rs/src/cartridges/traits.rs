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
    /// at least `one rule` must pass the test for at least `one match`
    fn any_r_for_any_m(&mut self) -> Self::CartridgeType;
}
