mod validate_bytes;
mod validate_str;

use super::base_error::PystvalError;
use super::cartridges::CartridgeBase;
use super::message::filling_message;
use super::rules::next::NextStep;
use super::rules::traits::RuleBase;
use crate::prelude::{Rule, RuleBytes};
use async_trait::async_trait;
use log::trace;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait ValidatorBase<R, C, IC, D>
where
    R: RuleBase,
    C: CartridgeBase<R, D> + Debug,
    IC: IntoIterator<Item = C>,
    D: PartialEq + Eq + Hash + Debug,
{
    fn new(cartridges: IC) -> Self;
    fn validate(&self, data: D) -> Result<(), Vec<PystvalError>>;
}

// Определите новый трейт для асинхронной валидации
#[async_trait]
pub trait ValidatorBaseAsync<R, C, IC, D>
where
    R: RuleBase,
    C: CartridgeBase<R, D> + Debug,
    IC: IntoIterator<Item = C>,
    D: PartialEq + Eq + Hash + Debug,
{
    async fn async_validate(&self, data: D) -> Result<(), Vec<PystvalError>>;
}

pub struct TemplateValidator<IC, D>
where
    D: PartialEq + Eq + Hash + Debug,
{
    cartridges: IC,
    _phantom: PhantomData<D>,
}
