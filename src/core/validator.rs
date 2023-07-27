use super::cartridges::CartridgeBase;
use super::rules::traits::RuleBase;
use crate::prelude::*;
use async_trait::async_trait;
use std::hash::Hash;

pub trait ValidatorBase<R, I, C, D>
where
    R: RuleBase,
    I: IntoIterator<Item = R>,
    C: IntoIterator<Item = CartridgeRule<R, I>>, // Исправление: Здесь используем CartridgeRule
    D: PartialEq + Eq + Hash,
{
    fn new(cartridges: C) -> Self;
    fn validate(&self, data: D) -> Box<dyn CartridgeBase<R, I>>;
    // async fn async_validate(&self, data: D) -> Box<dyn CartridgeBase<R, I>>;
}

pub struct TemplateValidator<R, I, C>
where
    R: RuleBase,
    I: IntoIterator<Item = R>,
    C: IntoIterator<Item = CartridgeRule<R, I>>, // Исправление: Здесь используем CartridgeRule
{
    cartridges: C,
}

impl<I, C> ValidatorBase<Rule, I, C, &str> for TemplateValidator<Rule, I, C>
where
    I: IntoIterator<Item = Rule>,
    C: IntoIterator<Item = CartridgeRule<Rule, I>>, // Исправление: Здесь используем CartridgeRule
{
    fn new(cartridges: C) -> Self {
        Self { cartridges }
    }

    fn validate(&self, data: &str) -> Box<dyn CartridgeBase<Rule, I>> {
        todo!()
    }

    // async fn async_validate(&self, data: &str) -> Box<dyn CartridgeBase<&str, I>> {
    //     todo!()
    // }
}

impl<I, C> ValidatorBase<RuleBytes, I, C, &[u8]> for TemplateValidator<RuleBytes, I, C>
where
    I: IntoIterator<Item = RuleBytes>,
    C: IntoIterator<Item = CartridgeRule<RuleBytes, I>>, // Исправление: Здесь используем CartridgeRule
{
    fn new(cartridges: C) -> Self {
        Self { cartridges }
    }

    fn validate(&self, data: &[u8]) -> Box<dyn CartridgeBase<RuleBytes, I>> {
        todo!()
    }
    // async fn async_validate(&self, data: &[u8]) -> Box<dyn CartridgeBase<[&u8], I>> {
    //     todo!()
    // }
}
