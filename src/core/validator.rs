use super::cartridges::{self, CartridgeBase};
use super::rules::next::NextStep;
use super::rules::traits::RuleBase;
use crate::prelude::{Rule, RuleBytes};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
pub trait ValidatorBase<R, C, IC, D>
where
    R: RuleBase,
    C: CartridgeBase<R, D>,
    IC: IntoIterator<Item = C>,
    D: PartialEq + Eq + Hash + Debug,
{
    fn new(cartridges: IC) -> Self;
    fn validate(&self, data: &D) -> Option<Vec<Box<dyn CartridgeBase<R, D>>>>;
}

pub struct TemplateValidator<R, C, IC, D>
where
    R: RuleBase,
    C: CartridgeBase<R, D>,
    IC: IntoIterator<Item = C>,
    D: PartialEq + Eq + Hash + Debug,
{
    cartridges: IC,
    phantom_data: PhantomData<(R, D)>,
}
impl<'a, C, IC> ValidatorBase<Rule, C, IC, &'a str> for TemplateValidator<Rule, C, IC, &'a str>
where
    C: CartridgeBase<Rule, &'a str>,
    IC: IntoIterator<Item = C> + AsRef<[C]>,
{
    fn new(cartridges: IC) -> Self {
        Self {
            cartridges,
            phantom_data: PhantomData,
        }
    }

    fn validate(&self, data: &&'a str) -> Option<Vec<Box<dyn CartridgeBase<Rule, &'a str>>>> {
        let mut result: Option<Vec<Box<dyn CartridgeBase<Rule, &'a str>>>> = Some(Vec::new());
        for cartridge in self.cartridges.as_ref().into_iter() {
            if let NextStep::Error(err) = cartridge.run(data) {
                
            }
        }
        result
    }
}

// #[test]
// fn x() {
//     let cartridge_1 = Cartridge::new(
//         1,
//         "the error message from `cartridge_1`",
//         [Rule::new(r".+", MatchRequirement::MustBeFound)],
//     );

//     let cartridge_2 = Cartridge::new(
//         1,
//         "the error message from `cartridge_2`",
//         [Rule::new(r"\d+", MatchRequirement::MustNotBeFound)],
//     );
//     let cartridge_bytes = CartridgeRuleBytes::new(
//         1,
//         "the error message from `cartridge_bytes`",
//         [RuleBytes::new(r".+", MatchRequirement::MustBeFound)],
//     );
//     // let
//     let validator_1 = TemplateValidator::new([cartridge_1, cartridge_2]);
//     let validator_2 = TemplateValidator::new([cartridge_bytes]);
// }
