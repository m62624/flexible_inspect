use super::rules::traits::RuleBase;
use crate::prelude::*;

pub trait CartridgeBase<T: RuleBase, I: IntoIterator<Item = T>> {
    fn id(&mut self) -> &mut i64;
    fn message(&mut self) -> &mut String;
    fn rules(&self) -> &I;
}

pub struct CartridgeRule<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    id: i64,
    message: String,
    rules: I,
}

pub struct CartridgeRuleBytes<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    id: i64,
    message: String,
    rules: I,
}

impl<I> CartridgeRule<Rule, I>
where
    I: IntoIterator<Item = Rule>,
{
    pub fn new<S: Into<String>>(id: i64, message: S, rules: I) -> Self {
        Self {
            id,
            message: message.into(),
            rules,
        }
    }
}

impl<I> CartridgeRuleBytes<RuleBytes, I>
where
    I: IntoIterator<Item = RuleBytes>,
{
    pub fn new<S: Into<String>>(id: i64, message: S, rules: I) -> Self {
        Self {
            id,
            message: message.into(),
            rules,
        }
    }
}

impl<'a, I> CartridgeBase<Rule, I> for CartridgeRule<Rule, I>
where
    I: IntoIterator<Item = Rule>,
{
    fn id(&mut self) -> &mut i64 {
        &mut self.id
    }

    fn message(&mut self) -> &mut String {
        &mut self.message
    }

    fn rules(&self) -> &I {
        &self.rules
    }
}

impl<'a, I> CartridgeBase<RuleBytes, I> for CartridgeRuleBytes<RuleBytes, I>
where
    I: IntoIterator<Item = RuleBytes>,
{
    fn id(&mut self) -> &mut i64 {
        &mut self.id
    }

    fn message(&mut self) -> &mut String {
        &mut self.message
    }

    fn rules(&self) -> &I {
        &self.rules
    }
}

#[test]
fn x() {
    let mut cartridge = CartridgeRule::new(
        0,
        "message",
        [Rule::new(r"\d+", MatchRequirement::MustNotBeFound)],
    );
    let x = cartridge.rules();
}
