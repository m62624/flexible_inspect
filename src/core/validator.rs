use super::cartridges::CartridgeBase;
use super::rules::traits::RuleBase;
use crate::prelude::*;
use std::hash::Hash;
pub trait ValidatorBase<T, I, D>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
    D: PartialEq + Eq + Hash,
{
    fn new(cartridges: I) -> Self;
    fn validate(&self, data: D) -> Box<dyn CartridgeBase<T, I>>;
}

pub struct TemplateValidator<T, I>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
{
    cartridges: I,
}

impl<I> ValidatorBase<Rule, I, &str> for TemplateValidator<Rule, I>
where
    I: IntoIterator<Item = Rule>,
{
    fn new(cartridges: I) -> Self {
        Self { cartridges }
    }

    fn validate(&self, data: &str) -> Box<dyn CartridgeBase<Rule, I>> {
        todo!()
    }
}

impl<I> ValidatorBase<RuleBytes, I, &[u8]> for TemplateValidator<RuleBytes, I>
where
    I: IntoIterator<Item = RuleBytes>,
{
    fn new(cartridges: I) -> Self {
        Self { cartridges }
    }

    fn validate(&self, data: &[u8]) -> Box<dyn CartridgeBase<RuleBytes, I>> {
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
    // let validator_1 = TemplateValidator::new(vec![cartridge_1]);
}
