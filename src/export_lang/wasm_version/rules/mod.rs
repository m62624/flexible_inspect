pub mod rule_bytes;
pub mod rule_str;
use super::*;
use crate::prelude::MatchRequirement as RustMatchRequirement;
use crate::prelude::{Rule, RuleBytes};
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub enum MatchRequirement {
    MustBeFound = 0,
    MustNotBeFound = 1,
}

impl From<MatchRequirement> for RustMatchRequirement {
    fn from(value: MatchRequirement) -> Self {
        match value {
            MatchRequirement::MustBeFound => RustMatchRequirement::MustBeFound,
            MatchRequirement::MustNotBeFound => RustMatchRequirement::MustNotBeFound,
        }
    }
}

pub trait WasmRuleModifiers {
    type WasmRuleType;
    /// modifier to set the match counter, condition counter == match
    fn _counter_is_equal(&mut self, count: usize) -> Self::WasmRuleType;
    fn _counter_more_than(&mut self, count: usize) -> Self::WasmRuleType;
    fn _counter_less_than(&mut self, count: usize) -> Self::WasmRuleType;

    fn _mode_all_rules_for_at_least_one_match(&mut self) -> Self::WasmRuleType;
    fn _mode_at_least_one_rule_for_all_matches(&mut self) -> Self::WasmRuleType;
    fn _mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::WasmRuleType;
}
