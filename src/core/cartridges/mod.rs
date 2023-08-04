// =======================================================
mod cartridges_bytes;
mod cartridges_str;
mod root_mode_matching;
// =======================================================
use super::rules::{self, next::NextStep, traits::RuleBase, CaptureData};
use crate::prelude::*;
#[cfg(any(feature = "serde", feature = "wasm"))]
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Debug, hash::Hash};
// =======================================================

/// This trait is required for single access to `Rule cartridges` or `RuleBytes cartridges`
pub trait CartridgeBase<T, D>
where
    T: RuleBase,
    D: PartialEq + Eq + Hash + Debug,
{
    /// Run the validation for one `cartridge`
    fn run(&self, data: D) -> NextStep;
    /// Get the `error code`
    fn get_id(&self) -> i32;
    /// Get an `error message` with data
    fn get_message(&self) -> &str;
}

/// The container structure for `custom rules`, `error message` and `error code`.\
/// Use a container for one object if possible. Imagine that one container is one specific error `NotFound`, `InvalidHeader`, `WrongCase`.\
/// ( *Each cartridge can only hold one type at a time, `Rule` or `RuleBytes`* )
/// **by default, all rules must pass every match check**
#[cfg_attr(
    any(feature = "serde", feature = "wasm"),
    derive(Serialize, Deserialize)
)]
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
    pub fn new<S, I>(id: i32, message: S, rules: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            root_rule: T::_new("SYSTEM_ROOT_RULE", MatchRequirement::MustBeFound).extend(rules),
            id,
            message: message.into(),
        }
    }
}
