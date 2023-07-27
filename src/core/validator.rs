use super::{
    cartridges::{self, CartridgeBase},
    rules::traits::RuleBase,
};
use crate::prelude::*;
pub trait ValidatorBase<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    fn new(cartridges: I) -> Self;
    fn validate<S: AsRef<str>>(&self, text: S) -> Box<dyn CartridgeBase<T, I>>;
}

pub struct TemplateValidator<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    cartridges: I,
}

impl<I> ValidatorBase<Rule, I> for TemplateValidator<Rule, I>
where
    I: IntoIterator<Item = Rule>,
{
    fn new(cartridges: I) -> Self {
        Self { cartridges }
    }
    fn validate<S: AsRef<str>>(&self, text: S) -> Box<dyn CartridgeBase<Rule, I>> {
        todo!()
    }
}

impl<I> ValidatorBase<RuleBytes, I> for TemplateValidator<RuleBytes, I>
where
    I: IntoIterator<Item = RuleBytes>,
{
    fn new(cartridges: I) -> Self {
        Self { cartridges }
    }

    fn validate<S: AsRef<str>>(&self, text: S) -> Box<dyn CartridgeBase<RuleBytes, I>> {
        todo!()
    }
}

#[test]
fn x() {
    let cartridge_1 = CartridgeRule::new(
        1,
        "the error message from `cartridge_1`",
        vec![Rule::new(r".+", MatchRequirement::MustBeFound)],
    );
    let validator_1 = TemplateValidator::new(vec![cartridge_1]);
}
