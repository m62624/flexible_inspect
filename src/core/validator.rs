// use super::cartridges::CartridgeBase;
// use super::rules::traits::RuleBase;
// use crate::prelude::*;
// use async_trait::async_trait;
// use std::hash::Hash;
// use std::marker::PhantomData;
// pub trait ValidatorBase<R, I, C, IC, D>
// where
//     R: RuleBase,
//     I: IntoIterator<Item = R>,
//     C: CartridgeBase<R, I>,
//     IC: IntoIterator<Item = C>,
//     D: PartialEq + Eq + Hash,
// {
//     fn new(cartridges: IC) -> Self;
//     fn validate(&self, data: D) -> Box<dyn CartridgeBase<R, I>>;
// }

// pub struct TemplateValidator<R, I, C, IC>
// where
//     R: RuleBase,
//     I: IntoIterator<Item = R>,
//     C: CartridgeBase<R, I>,
//     IC: IntoIterator<Item = C>,
// {
//     cartridge: IC,
//     phantom_data: PhantomData<(R, I)>,
// }

// impl<I, IC> ValidatorBase<Rule, I, CartridgeRule<Rule, I>, IC, &str>
//     for TemplateValidator<Rule, I, CartridgeRule<Rule, I>, IC>
// where
//     I: IntoIterator<Item = Rule>,
//     IC: IntoIterator<Item = CartridgeRule<Rule, I>>,
// {
//     fn new(cartridges: IC) -> Self {
//         todo!()
//     }

//     fn validate(&self, data: &str) -> Box<dyn CartridgeBase<Rule, I>> {
//         todo!()
//     }

//     // async fn async_validate(&self, data: &str) -> Box<dyn CartridgeBase<&str, I>> {
//     //     todo!()
//     // }
// }

// impl<I, IC> ValidatorBase<RuleBytes, I, CartridgeRuleBytes<RuleBytes, I>, IC, &[u8]>
//     for TemplateValidator<RuleBytes, I, CartridgeRuleBytes<RuleBytes, I>, IC>
// where
//     I: IntoIterator<Item = RuleBytes>,
//     IC: IntoIterator<Item = CartridgeRuleBytes<RuleBytes, I>>,
// {
//     fn new(cartridges: IC) -> Self {
//         todo!()
//     }

//     fn validate(&self, data: &[u8]) -> Box<dyn CartridgeBase<RuleBytes, I>> {
//         todo!()
//     }
// }

// #[test]
// fn x() {
//     let cartridge_1 = CartridgeRule::new(
//         1,
//         "the error message from `cartridge_1`",
//         [Rule::new(r".+", MatchRequirement::MustBeFound)],
//     );

//     let cartridge_2 = CartridgeRule::new(
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
