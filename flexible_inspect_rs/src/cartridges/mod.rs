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
    /// # Example:
    /// ```rust
    /// # use flexible_inspect_rs::prelude::*;
    ///      let cartridge = Cartridge::new(
    ///         0,
    ///         "Secret key not found",
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
    /// * Each cartridge supports filling a message with unwanted data, when you specify a message, you can specify a variable in the message in the format : **`{variable}`**. After specifying the identical group name in any rule
    /// # Example:
    /// 
    pub fn new<S, I>(id: i32, message: S, rules: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            root_rule: T::_new("SYSTEM_ROOT_RULE", MatchRequirement::MustBeFound)
                .all_r_for_any_m()
                .extend(rules),
            id,
            message: message.into(),
        }
    }
}
