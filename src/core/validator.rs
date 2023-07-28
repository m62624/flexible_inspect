use super::cartridges::{self, Cartridge, CartridgeBase};
use super::rules::next::NextStep;
use super::rules::traits::RuleBase;
use super::rules::MatchRequirement;
use crate::prelude::{Rule, RuleBytes};
use std::fmt::Debug;
use std::hash::Hash;
pub trait ValidatorBase<R, C, IC, D>
where
    R: RuleBase,
    C: CartridgeBase<R, D>,
    IC: IntoIterator<Item = C>,
    D: PartialEq + Eq + Hash + Debug,
{
    fn new(cartridges: IC) -> Self;
    fn validate(&self, data: D) -> Option<Vec<Box<dyn CartridgeBase<R, D>>>>;
}

pub struct TemplateValidator<IC> {
    cartridges: IC,
}

impl<'a, C, IC> ValidatorBase<Rule, C, IC, &'a str> for TemplateValidator<IC>
where
    C: CartridgeBase<Rule, &'a str>,
    IC: IntoIterator<Item = C> + AsRef<[C]>,
{
    fn new(cartridges: IC) -> Self {
        Self { cartridges }
    }

    fn validate(&self, data: &'a str) -> Option<Vec<Box<dyn CartridgeBase<Rule, &'a str>>>> {
        let mut result: Option<Vec<Box<dyn CartridgeBase<Rule, &'a str>>>> = Some(Vec::new());
        for cartridge in self.cartridges.as_ref().into_iter() {
            if let NextStep::Error(err) = cartridge.run(data) {
                todo!()
            }
        }
        result
    }
}

impl<'a, C, IC> ValidatorBase<RuleBytes, C, IC, &'a [u8]> for TemplateValidator<IC>
where
    C: CartridgeBase<RuleBytes, &'a [u8]>,
    IC: IntoIterator<Item = C> + AsRef<[C]>,
{
    fn new(cartridges: IC) -> Self {
        Self { cartridges }
    }

    fn validate(&self, data: &'a [u8]) -> Option<Vec<Box<dyn CartridgeBase<RuleBytes, &'a [u8]>>>> {
        let mut result: Option<Vec<Box<dyn CartridgeBase<RuleBytes, &'a [u8]>>>> = Some(Vec::new());
        for cartridge in self.cartridges.as_ref().into_iter() {
            if let NextStep::Error(err) = cartridge.run(data) {
                todo!()
            }
        }
        result
    }
}

#[test]
fn x() {
    let cartridge_1 = Cartridge::new(
        1,
        "the error message from `cartridge_1`",
        [Rule::new(r".+", MatchRequirement::MustBeFound)],
    );

    let cartridge_2 = Cartridge::new(
        1,
        "the error message from `cartridge_2`",
        [Rule::new(r"\d+", MatchRequirement::MustNotBeFound)],
    );

    // let
    let validator_1 = TemplateValidator::new([cartridge_1, cartridge_2]);
    validator_1.validate("x");
}
