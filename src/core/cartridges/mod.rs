mod cartridges_bytes;
mod cartridges_str;
use super::rules::{self, next::NextStep, traits::RuleBase, CaptureData};
use crate::prelude::*;
use std::{collections::HashSet, fmt::Debug, hash::Hash, sync::Arc};

/// This trait is required for single access to `Rule cartridges` or `RuleBytes cartridges`
pub trait CartridgeBase<T, I, D>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
    D: PartialEq + Eq + Hash + Debug,
{
    /// Based on the received error code, you can implement your own actions
    fn id(&self) -> i64;
    /// Error messages, with the possibility of outputting additional data
    fn message(&mut self) -> &mut String;
    /// Rules for validation
    fn rules(&self) -> &Option<I>;
    fn run(&mut self, data: D) -> NextStep;
}

/// Container for `rules` + `error message` + `error code`
#[derive(Debug)]
pub struct Cartridge<T, I>(Arc<TakeCartridgeForAsync<T, I>>)
where
    T: RuleBase,
    I: IntoIterator<Item = T>;

impl<T, I> Cartridge<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    pub fn new<S: Into<String>>(id: i64, message: S, rules: I) -> Self {
        Self(Arc::new(TakeCartridgeForAsync::new(id, message, rules)))
    }
}

/// This structure is needed to pass to the async task
#[derive(Debug, Default)]
pub struct TakeCartridgeForAsync<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    root_rule: Option<T>,
    id: i64,
    message: String,
    rules: Option<I>,
}

impl<T, I> TakeCartridgeForAsync<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    pub fn new<S: Into<String>>(id: i64, message: S, rules: I) -> Self {
        Self {
            root_rule: None,
            id,
            message: message.into(),
            rules: Some(rules),
        }
    }
}
