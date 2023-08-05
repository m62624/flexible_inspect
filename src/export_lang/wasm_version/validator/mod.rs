use async_trait::async_trait;

use super::*;
use crate::core::validator::ValidatorBase;
use crate::core::{cartridges::Cartridge, rules::rule_str::Rule, validator::TemplateValidator};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
mod validator_bytes;
mod validator_str;

#[async_trait]
pub trait WasmValidatorBase<D: Debug + Hash + PartialEq> {
    fn validate(&self, data: D) -> Option<Vec<JsValue>>;
    async fn async_validate(&self, data: D) -> Result<JsValue, JsValue>;
}
