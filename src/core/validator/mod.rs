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

pub struct TemplateValidator<IC> {
    cartridges: IC,
}
