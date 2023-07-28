mod cartridges_bytes;
mod cartridges_str;
use super::rules::{self, next::NextStep, traits::RuleBase, CaptureData};
use crate::prelude::*;
use std::{collections::HashSet, fmt::Debug, hash::Hash, sync::Arc};

/// This trait is required for single access to `Rule cartridges` or `RuleBytes cartridges`
pub trait CartridgeBase<T, I, D>
where
    T: RuleBase,
    D: PartialEq + Eq + Hash + Debug,
{
    /// Based on the received error code, you can implement your own actions
    fn id(&self) -> i64;
    /// Error messages, with the possibility of outputting additional data
    fn message(&mut self) -> &mut String;
    /// Rules for validation
    fn root_rule(&self) -> &T;
    fn run(&mut self, data: D) -> NextStep;
}

/// Container for `rules` + `error message` + `error code`
#[derive(Debug)]
pub struct Cartridge<T>(Arc<TakeCartridgeForAsync<T>>)
where
    T: RuleBase + RuleModifiers;

impl<T> Cartridge<T>
where
    T: RuleBase + RuleModifiers<RuleType = T>,
{
    pub fn new<S, I>(id: i64, message: S, rules: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self(Arc::new(TakeCartridgeForAsync::new(id, message, rules)))
    }
}

/// This structure is needed to pass to the async task
#[derive(Debug, Default)]
pub struct TakeCartridgeForAsync<T>
where
    T: RuleBase,
{
    root_rule: T,
    id: i64,
    message: String,
}

impl<T> TakeCartridgeForAsync<T>
where
    T: RuleBase + RuleModifiers<RuleType = T>,
{
    pub fn new<S, I>(id: i64, message: S, rules: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            root_rule: T::new("SYSTEM_ROOT_RULE", MatchRequirement::MustBeFound).extend(rules),
            id,
            message: message.into(),
        }
    }
}
