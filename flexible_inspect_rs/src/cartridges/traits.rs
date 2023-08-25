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

/// This trait requires implementations of the most basic methods for any `Cartridge`.
pub trait CartridgeModifiers {
    /// The type of the cartridge that will be returned after applying the modifier
    type CartridgeType;

    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least one rule must pass at least one match check
    fn any_r_for_any_m(self) -> Self::CartridgeType;
}
