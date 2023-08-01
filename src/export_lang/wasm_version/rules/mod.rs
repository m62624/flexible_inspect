mod rule_bytes;
mod rule_str;
use super::*;
use crate::prelude::MatchRequirement as RustMatchRequirement;
use crate::prelude::{Rule, RuleBytes};

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
    type RustRuleType;
    /// modifier to set the match counter, condition counter == match
    fn _counter_is_equal(&mut self, count: usize) -> Self::WasmRuleType;
    fn _counter_more_than(&mut self, count: usize) -> Self::WasmRuleType;

    fn _counter_less_than(&mut self, count: usize) -> Self::WasmRuleType;
    fn _mode_all_rules_for_at_least_one_match(&mut self) -> Self::WasmRuleType;
    fn _mode_at_least_one_rule_for_all_matches(&mut self) -> Self::WasmRuleType;
    fn _mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self::WasmRuleType;
    fn _to_rust_for_extend<V, T: Into<V>>(nested_rules: Vec<T>) -> Vec<Self::RustRuleType>
    where
        Vec<<Self as WasmRuleModifiers>::RustRuleType>: FromIterator<V>,
    {
        nested_rules.into_iter().map(|rule| rule.into()).collect()
    }
}
