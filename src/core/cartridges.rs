use super::rules::traits::RuleBase;
use crate::{prelude::Rule, prelude::RuleBytes};

pub trait CartridgeBase<'a, T: RuleBase> {
    fn id(&mut self) -> i64;
    fn message(&mut self) -> &String;
}

pub struct CartridgeRule<T: IntoIterator<Item = Rule>> {
    id: i64,
    message: String,
    rules: T,
}

pub struct CartridgeRuleBytes<T: IntoIterator<Item = RuleBytes>> {
    id: i64,
    message: String,
    rules: T,
}

impl<T: IntoIterator<Item = Rule>> CartridgeRule<T> {
    pub fn new<S: Into<String>>(id: i64, message: S, rules: T) -> Self {
        Self {
            id,
            message: message.into(),
            rules,
        }
    }
}

impl<T: IntoIterator<Item = RuleBytes>> CartridgeRuleBytes<T> {
    pub fn new<S: Into<String>>(id: i64, message: S, rules: T) -> Self {
        Self {
            id,
            message: message.into(),
            rules,
        }
    }
}

impl<'a, T: IntoIterator<Item = Rule> + AsRef<&'a T> + 'a> CartridgeBase<'a, Rule>
    for CartridgeRule<T>
{
    fn id(&mut self) -> i64 {
        self.id
    }

    fn message(&mut self) -> &String {
        &self.message
    }
}

#[test]
fn x() {
    let rule = Rule::new("x", super::rules::MatchRequirement::MustBeFound);
    let cartridge = CartridgeRule::new(1, "x", [rule]);
    let s = cartridge.rules.as_ref();
    cartridge.rules.as_ref().into_iter().for_each(|rule| {
        println!("{:#?}", rule);
    });
}
