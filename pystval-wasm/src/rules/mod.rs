mod rule_bytes;
mod rule_str;
use super::*;

#[wasm_bindgen(js_name = "MatchRequirement")]
pub enum WasmMatchRequirement {
    MustBeFound = 0,
    MustNotBeFound = 1,
}

impl From<WasmMatchRequirement> for MatchRequirement {
    fn from(value: WasmMatchRequirement) -> Self {
        match value {
            WasmMatchRequirement::MustBeFound => MatchRequirement::MustBeFound,
            WasmMatchRequirement::MustNotBeFound => MatchRequirement::MustNotBeFound,
        }
    }
}
