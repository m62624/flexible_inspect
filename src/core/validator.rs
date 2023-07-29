use super::base_error::PystvalError;
use super::cartridges::CartridgeBase;
use super::message::filling_message;
use super::rules::next::NextStep;
use super::rules::traits::RuleBase;
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
    fn validate(&self, data: D) -> Result<(), Vec<PystvalError>>;
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

    fn validate(&self, data: &'a str) -> Result<(), Vec<PystvalError>> {
        let mut error = Vec::new();
        for cartridge in self.cartridges.as_ref().iter() {
            if let NextStep::Error(extra_with_value) = cartridge.run(data) {
                error.push(PystvalError::new(
                    cartridge.get_id(),
                    filling_message(cartridge.get_message(), extra_with_value),
                ));
            }
        }
        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
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

    fn validate(&self, data: &'a [u8]) -> Result<(), Vec<PystvalError>> {
        let mut error = Vec::new();
        for cartridge in self.cartridges.as_ref().iter() {
            if let NextStep::Error(extra_with_value) = cartridge.run(data) {
                error.push(PystvalError::new(
                    cartridge.get_id(),
                    filling_message(cartridge.get_message(), extra_with_value),
                ));
            }
        }
        if error.is_empty() {
            Ok(())
        } else {
            Err(error)
        }
    }
}
