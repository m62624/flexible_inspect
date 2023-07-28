use super::rules::traits::RuleBase;
use crate::prelude::*;
use std::sync::Arc;

/// This trait is required for single access to `Rule cartridges` or `RuleBytes cartridges`
pub trait CartridgeBase<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    /// Based on the received error code, you can implement your own actions
    fn id(&self) -> i64;
    /// Error messages, with the possibility of outputting additional data
    fn message(&mut self) -> &mut String;
    /// Rules for validation
    fn rules(&self) -> &I;
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
#[derive(Debug)]
pub struct TakeCartridgeForAsync<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    id: i64,
    message: String,
    rules: I,
}

impl<T, I> TakeCartridgeForAsync<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    pub fn new<S: Into<String>>(id: i64, message: S, rules: I) -> Self {
        Self {
            id,
            message: message.into(),
            rules,
        }
    }
}

impl<I> CartridgeBase<Rule, I> for TakeCartridgeForAsync<Rule, I>
where
    I: IntoIterator<Item = Rule>,
{
    fn id(&self) -> i64 {
        self.id
    }

    fn message(&mut self) -> &mut String {
        &mut self.message
    }

    fn rules(&self) -> &I {
        &self.rules
    }
}

impl<I> CartridgeBase<RuleBytes, I> for TakeCartridgeForAsync<RuleBytes, I>
where
    I: IntoIterator<Item = RuleBytes>,
{
    fn id(&self) -> i64 {
        self.id
    }

    fn message(&mut self) -> &mut String {
        &mut self.message
    }

    fn rules(&self) -> &I {
        &self.rules
    }
}
