// =======================================================
mod validate_bytes;
mod validate_str;
use crate::cartridges::traits::CartridgeBase;
use crate::error::ValidationError;
// =======================================================
#[cfg(feature = "serde")]
use super::{Deserialize, Serialize};
use crate::rules::next::NextStep;
use async_trait::async_trait;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
#[cfg(feature = "export_to_other_languages")]
use std::sync::Arc;

/// Use trait for `overloading` methods of `&str` and `&[u8]` types
#[async_trait]
pub trait ValidatorBase<C, IC, D>
where
    C: CartridgeBase<D> + Sync + Debug,
    IC: IntoIterator<Item = C> + Sync + Send,
    D: PartialEq + Eq + Hash + Debug,
{
    /// Constructor for creating a validator
    /// Load different [`cartridges`](crate::prelude::Cartridge)  into the `construcrtor` to create a validator for different situations
    /// # Example:
    /// ```rust ignore
    /// # use flexible_inspect_rs::prelude::*;
    /// // Some Cartridges
    /// let validator_for_web = TemplateValidator::new(vec![Not_Found, Conflict_File, ...]);
    /// let validator_for_linux = TemplateValidator::new(vec![Forbidden_Sudo, Invalid_Path, ...]);
    /// let validator_for_file = TemplateValidator::new(vec![Incorrect_Header, ...]);
    /// ```
    fn new(cartridges: IC) -> Self;
    /// Method for validating data
    ///
    /// # Example:
    /// A simple script file where we check that it is a `bash script`,
    /// check that there are only five `if` fragments, and also no `sudo` | `su` commands.
    /// ```rust
    /// # use flexible_inspect_rs::prelude::*;
    /// let bash_script = r###"
    /// #!/bin/bash
    /// if [ $# -lt 1 ]; then
    ///     echo "Usage: $0 <number> <word> <username>"
    ///     exit 1
    /// fi
    /// if [ "$1" -eq "$1" ] 2>/dev/null; then
    ///     for ((i=0; i<$1; i++)); do
    ///         echo "Short message for user $3"
    ///     done
    /// fi
    /// if [ -n "$2" ]; then
    ///     echo "Word: $2"
    /// fi
    /// if [ -n "$3" ]; then
    ///     echo "Warning: Using sudo_su as $3 can be dangerous"
    ///     sudo rm 1234_important_file.txt
    /// fi
    /// if [[ -n "$4" && "$4" =~ ^[0-9]{8,}$ ]]; then
    ///     echo "Password set: $4"
    /// fi        
    /// "###;
    /// let forbidden_sudo = Cartridge::new(
    ///             0,
    ///             "Scripts with increased access are forbidden : {SUDO_OR_SU}",
    ///             [
    ///                 Rule::new(r"(?P<SUDO_OR_SU>sudo .+)", MatchRequirement::MustNotBeFound),
    ///                 Rule::new(r"(?P<SUDO_OR_SU>su .+)", MatchRequirement::MustNotBeFound),
    ///             ],
    ///         );
    /// let incorrect_header = Cartridge::new(
    ///             1,
    ///             "Inccorect type file",
    ///             [Rule::new(
    ///                 r"^\n?#!/bin/bash\s+?\n?",
    ///                 MatchRequirement::MustBeFound,
    ///             )],
    ///         );
    /// let incorrect_body = Cartridge::new(
    ///             2,
    ///             "Inccorect fragment code",
    ///             [
    ///                 Rule::new(r"(?s).+", MatchRequirement::MustBeFound).extend([Rule::new(
    ///                     r"if.+then",
    ///                     MatchRequirement::MustBeFound,
    ///                 )
    ///                 .counter_is_equal(5)]),
    ///             ],
    ///         );
    /// let validator_bash = TemplateValidator::new(vec![
    ///             forbidden_sudo,
    ///             incorrect_header,
    ///             incorrect_body
    ///         ]);
    /// if let Err(errors) = validator_bash.validate(bash_script) {
    ///     for error in errors {
    ///         println!("{}", error);
    ///        }
    ///     }
    /// ```
    /// # Output:
    /// > **0 - Scripts with increased access are forbidden : sudo rm 1234_important_file.txt**
    fn validate(&self, data: D) -> Result<(), Vec<ValidationError>>;
    /// Method for validating data. Runs validation of each cartridge asynchronously
    ///
    /// # Example:
    /// A simple script file where we check that it is a `bash script`,
    /// check that there are only five `if` fragments, and also no `sudo` | `su` commands.
    /// ```rust
    /// # use flexible_inspect_rs::prelude::*;
    /// let bash_script = r###"
    /// #!/bin/bash
    /// if [ $# -lt 1 ]; then
    ///     echo "Usage: $0 <number> <word> <username>"
    ///     exit 1
    /// fi
    /// if [ "$1" -eq "$1" ] 2>/dev/null; then
    ///     for ((i=0; i<$1; i++)); do
    ///         echo "Short message for user $3"
    ///     done
    /// fi
    /// if [ -n "$2" ]; then
    ///     echo "Word: $2"
    /// fi
    /// if [ -n "$3" ]; then
    ///     echo "Warning: Using sudo_su as $3 can be dangerous"
    ///     sudo rm 1234_important_file.txt
    /// fi
    /// if [[ -n "$4" && "$4" =~ ^[0-9]{8,}$ ]]; then
    ///     echo "Password set: $4"
    /// fi        
    /// "###;
    /// let forbidden_sudo = Cartridge::new(
    ///             0,
    ///             "Scripts with increased access are forbidden : {SUDO_OR_SU}",
    ///             [
    ///                 Rule::new(r"(?P<SUDO_OR_SU>sudo .+)", MatchRequirement::MustNotBeFound),
    ///                 Rule::new(r"(?P<SUDO_OR_SU>su .+)", MatchRequirement::MustNotBeFound),
    ///             ],
    ///         );
    /// let incorrect_header = Cartridge::new(
    ///             1,
    ///             "Inccorect type file",
    ///             [Rule::new(
    ///                 r"^\n?#!/bin/bash\s+?\n?",
    ///                 MatchRequirement::MustBeFound,
    ///             )],
    ///         );
    /// let incorrect_body = Cartridge::new(
    ///             2,
    ///             "Inccorect fragment code",
    ///             [
    ///                 Rule::new(r"(?s).+", MatchRequirement::MustBeFound).extend([Rule::new(
    ///                     r"if.+then",
    ///                     MatchRequirement::MustBeFound,
    ///                 )
    ///                 .counter_is_equal(5)]),
    ///             ],
    ///         );
    /// let validator_bash = TemplateValidator::new(vec![
    ///             forbidden_sudo,
    ///             incorrect_header,
    ///             incorrect_body
    ///         ]);
    /// if let Err(errors) = validator_bash.async_validate(bash_script).await {
    ///     for error in errors {
    ///         println!("{}", error);
    ///        }
    ///     }
    /// ```
    /// # Output:
    /// > **0 - Scripts with increased access are forbidden : sudo rm 1234_important_file.txt**
    async fn async_validate(&self, data: D) -> Result<(), Vec<ValidationError>>;
}

/// The structure for creating unique validators, load different `cartridges` to validate data.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateValidator<IC, D>
where
    D: PartialEq + Eq + Hash + Debug,
{
    pub(crate) cartridges: IC,
    _phantom: PhantomData<D>,
}
