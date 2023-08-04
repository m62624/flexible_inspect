pub mod cartridges_bytes;
pub mod cartridges_str;
use super::*;
use crate::core::{cartridges::Cartridge, rules::rule_bytes::RuleBytes, rules::rule_str::Rule};
use crate::prelude_wasm::WasmRule;
use crate::prelude_wasm::WasmRuleBytes;
use rules::WasmRuleModifiers;
use wasm_bindgen::JsValue;
