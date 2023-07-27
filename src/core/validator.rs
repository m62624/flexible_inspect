use super::cartridges::CartridgeBase;
use super::rules::traits::RuleBase;
use crate::prelude::*;
use std::hash::Hash;
pub trait ValidatorBase<T, I, C, D>
where
    T: RuleBase,
    I: IntoIterator<Item = T>,
    C: IntoIterator<Item = dyn CartridgeBase<T, I>>,
    D: PartialEq + Eq + Hash,
{
    fn new(cartridges: C) -> Self;
    fn validate(&self, data: D) -> Box<dyn CartridgeBase<T, I>>;
}

pub struct TemplateValidator<C, IC>
where
    IC: IntoIterator<Item = C>,
{
    cartridges: IC,
}

impl<I, C, IC> ValidatorBase<Rule, I, C, &str> for TemplateValidator<C, IC>
where
    I: IntoIterator<Item = Rule>,
    C: IntoIterator<Item = dyn CartridgeBase<Rule, I>>,
    IC: IntoIterator<Item = C>,
{
    fn new(cartridges: C) -> Self {
        todo!()
    }

    fn validate(&self, data: &str) -> Box<dyn CartridgeBase<Rule, I>> {
        todo!()
    }
}
