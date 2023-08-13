// =======================================================
mod cartridges_bytes;
mod cartridges_str;
pub mod traits;
// =======================================================
use super::rules::{self, next::NextStep, traits::RuleBase, CaptureData};
use crate::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "export_to_other_languages")]
use std::sync::Arc;
use std::{collections::HashSet, fmt::Debug, hash::Hash};
// =======================================================

/// The cartridge is the container of the rules.
///
/// **Notes:**
/// * Use a container for one object if possible. Imagine that one container is one specific error `NotFound`, `InvalidHeader`, `WrongCase`.
/// **by default, all rules must pass every match check**
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Cartridge<T>
where
    T: RuleBase,
{
    pub(crate) root_rule: T,
    pub(crate) id: i32,
    pub(crate) message: String,
}

impl<T> Cartridge<T>
where
    T: RuleBase + RuleModifiers<RuleType = T>,
{
    /// Constructor for `Cartridge`, *each cartridge can only hold one type at a time, `Rule` or `RuleBytes`*
    ///
    /// # Example:
    /// ```rust
    /// # use flexible_inspect_rs::prelude::*;
    ///  let cartridge = Cartridge::new(
    ///         0, // error code
    ///         "Secret key not found", // error message
    ///     [   
    ///         Rule::new(
    ///             r"d{3}-::x-al-xy-::\.d{5}[0-7]",
    ///             MatchRequirement::MustBeFound,
    ///         ),
    ///         Rule::new(
    ///             r"d{4}-::x-aG-xx-::\.d{5}[0-1]",
    ///             MatchRequirement::MustNotBeFound,
    ///         ),
    ///         Rule::new(
    ///             r"\[KEY - [a-z][a-z][a-z][0-9]\]",
    ///             MatchRequirement::MustBeFound,
    ///         ),
    ///     ],
    /// );
    /// ```
    /// **Notes**:
    /// * Each cartridge supports filling the message with unwanted data, when specifying a message,
    /// you can specify a variable in the message in the format : **`{variable}`**.
    /// After specifying an identical group name in any rule along with the *`MustNotBeFound`* modifier
    ///
    /// ## Example:
    /// ```rust
    /// # use flexible_inspect_rs::prelude::*;
    /// let cartridge = Cartridge::new(
    ///     1,
    ///     // Specify the same names to complete the message (INCORRECT_DATA)
    ///     "Incorrect command found `{INCORRECT_DATA}`",
    ///     [
    ///         // Check script, commands starting with `sudo` should not be found 
    ///         Rule::new(
    ///             // Specify the same names to complete the message (INCORRECT_DATA)
    ///             r"(?P<INCORRECT_DATA>sudo .+[^\n\s\.]?)",
    ///             // Specify the MustNotBeFound modifier
    ///             MatchRequirement::MustNotBeFound,
    ///         )
    ///     ],
    /// );
    ///
    /// // Safe example script
    /// let bash_script = r###"
    /// #!/bin/bash
    /// if [ "$1" = "--help" ]; then
    ///     echo "This is a sample script with --help option."
    ///     echo "Usage: ./myscript.sh [--help]"
    ///     echo "If you provide the --help option,
    ///     it will execute the dangerous command 'sudo rm -rf /'."
    ///     echo "Use with caution!"
    /// else
    ///     echo "Welcome to the safe area!"
    /// fi
    /// "###;
    ///
    /// // Create a unique validator to check the script
    /// let validator_for_linux_system = TemplateValidator::new([cartridge]);
    /// // Checking the result of validations
    /// if let Err(errors) = validator_for_linux_system.validate(bash_script) {
    ///     for error in errors {
    ///         println!("{}", error);
    ///     }
    /// }
    /// ```
    ///
    /// ## Output:
    /// > **1 - Incorrect command found `sudo rm -rf /'."`**
    ///
    /// * Fill in messages have a reserved variable to fill in `main_capture`, just specify this message in the cartridge messages and you don't have to specify a group in the rule
    ///
    /// ## Example:
    /// ```rust
    /// use flexible_inspect_rs::prelude::*;
    /// let cartridge = Cartridge::new(
    ///     1,
    ///     // Specify a reserved variable in messages
    ///     "Incorrect command found `{main_capture}`",
    ///     [
    ///         Rule::new(
    ///             // Check script, commands starting with `sudo` should not be found 
    ///             r"sudo .+[^\n\s\.]?",
    ///             // Specify the MustNotBeFound modifier
    ///             MatchRequirement::MustNotBeFound,
    ///         )
    ///     ],
    /// );
    /// ```
    ///
    /// ## Output:
    /// > **1 - Incorrect command found `sudo rm -rf /'."`**
    pub fn new<S, I>(id: i32, message: S, rules: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            root_rule: T::_new("SYSTEM_ROOT_RULE", MatchRequirement::MustBeFound)
                // .all_r_for_any_m()
                .extend(rules),
            id,
            message: message.into(),
        }
    }
}
