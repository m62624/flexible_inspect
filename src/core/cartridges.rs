use super::rules::traits::RuleBase;
use crate::prelude::*;

pub trait CartridgeBase<T: RuleBase, I: IntoIterator<Item = T>> {
    fn id(&mut self) -> &mut i64;
    fn message(&mut self) -> &mut String;
    fn rules(&self) -> &I;
}

pub struct Cartridge<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    id: i64,
    message: String,
    rules: I,
}

impl<T, I> Cartridge<T, I>
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

impl<'a, T, I> CartridgeBase<T, I> for Cartridge<T, I>
where
    T: RuleBase + 'a,
    I: IntoIterator<Item = T> + AsRef<&'a T> + 'a,
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
fn test_iter() {
    let mut cartridge_1 = Cartridge::new(
        -1,
        "this is error message",
        [Rule::new(r"\d+", MatchRequirement::MustNotBeFound)],
    );
    // cartridge_1.
}
