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

    /// modifier to change the rule matching mode,
    ///
    /// In this mode, at least one sub-rule should work for at least one match. If no sub-rule works on one of the matches, an error will be returned.
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError (Cartridge)
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///     |   [123], [456], [789]
    ///     |___ Subrule ".+" (MustBeFound) ---> [123] -- TRUE
    ///     |                                      |
    ///     |___ Subrule "\[\d+\]" (MustBeFound) __|
    ///     |___ Subrule "[a-z]+" (MustBeFound) ---> No Match -- TRUE (since other rules matched for at least one match)
    /// ```
    fn any_r_for_any_m(&mut self) -> Self::CartridgeType;
}
