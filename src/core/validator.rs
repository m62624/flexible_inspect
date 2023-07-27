use super::{
    cartridges::{self, CartridgeBase},
    rules::traits::RuleBase,
};
use crate::prelude::*;
pub trait ValidatorBase<T, I, U, IC>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
    U: CartridgeBase<T, I>,
    IC: IntoIterator<Item = U>,
{
    fn new(cartridges: IC) -> Self;
    fn validate<S: AsRef<str>>(&self, text: S) -> Box<dyn CartridgeBase<T, I>>;
}

pub struct TemplateValidator<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    cartridges: I,
}

impl<I, IC> ValidatorBase<Rule, I, CartridgeRule<Rule, I>, IC> for TemplateValidator<Rule, I>
where
    I: IntoIterator<Item = Rule>,
    IC: IntoIterator<Item = CartridgeRule<Rule, I>>,
{
    fn new(cartridges: IC) -> Self {
        Self { cartridges }
    }
    fn validate<S: AsRef<str>>(&self, text: S) -> Box<dyn CartridgeBase<Rule, I>> {
        todo!()
    }
}

impl<I, IC> ValidatorBase<RuleBytes, I, CartridgeRuleBytes<RuleBytes, I>, IC>
    for TemplateValidator<RuleBytes, I>
where
    I: IntoIterator<Item = RuleBytes>,
    IC: IntoIterator<Item = CartridgeRuleBytes<RuleBytes, I>>,
{
    fn new(cartridges: IC) -> Self {
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
