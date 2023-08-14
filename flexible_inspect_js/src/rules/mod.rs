pub mod rule_bytes;
pub mod rule_str;
use super::*;

/// The structure that defines what action is required when finding regular expression matches.
/// ## MatchRequirement
/// - `MustBeFound` - which means we must necessarily get a match from this regular expression
/// - `MustNotBeFound` - which means, based on this regular expression, we must not get a match
///
/// Behavior of `Rule` | `RuleBytes` under different conditions
///
/// | **MatchRequirement** | **Match found** | *does this rule have any subrules ?* | Result                         |
/// | ---------------- | ----------- | ---------------------------------- | ---------------------------------------- |
/// | MustBeFound      | Yes         | Yes                                | Continue processing subrules             |
/// | MustBeFound      | Yes         | No                                 | Finish processing                        |
/// | MustBeFound      | No          | Yes                                | Error (match must have been found)       |
/// | MustBeFound      | No          | No                                 | Error (match must have been found)       |
/// | MustNotBeFound   | Yes         | Yes                                | Continue processing subrules             |
/// | MustNotBeFound   | Yes         | No                                 | Error (match should not have been found) |
/// | MustNotBeFound   | No          | Yes                                | Finish processing                        |
/// | MustNotBeFound   | No          | No                                 | Finish processing                        |
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
