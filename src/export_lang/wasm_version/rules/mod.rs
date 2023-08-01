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
