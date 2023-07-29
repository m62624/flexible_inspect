use crate::core::rules::{rule_bytes::TakeRuleBytesForExtend, rule_str::TakeRuleForExtend};
use std::{fmt::Debug, hash::Hash};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Rule(Option<TakeRuleForExtend>);

#[wasm_bindgen]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RuleBytes(Option<TakeRuleBytesForExtend>);

#[wasm_bindgen]
pub struct TemplateValidator;
