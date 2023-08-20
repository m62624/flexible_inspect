mod modifiers;
use super::*;

#[wasm_bindgen(js_name = "RuleBytes")]
#[derive(Debug, Default, Serialize, Deserialize)]
/// A rule is the minimum unit of logic in a validator.
///
/// The most important feature is that the rule is recursive (don't worry, recursion is not used here).
/// Each rule can have nested rules, and these nested rules can have their own nested rules, and so on.
/// Thus, when the root rule is triggered, all the results obtained are passed to the nested rules, so you can build complex structural rules to suit any taste
///
/// # Notes
/// * To load a `RuleBytes` into a `CartridgeBytes`, always use `finish_build()` at the end, after applying modifiers or initializing the rule to prepare the value for processing in `Rust`
/// * Remember any modifier takes the contents of the `RuleBytes` body
/// and returns a new one with a changed parameter (only `None` from the original Rule remains),
/// so specify the modifier in the same place where you initialize `RuleBytes`.
/// * Use `&[u8]` when searching for regex matches in haystacks. ([**FancyRegex**](https://crates.io/crates/fancy-regex) capabilities are not available)
/// * Unicode support can be disabled, even if disabling it will result in a match with invalid `UTF-8` bytes. More info at [link](https://docs.rs/regex/latest/regex/bytes/index.html)
/// * How is recursive structure checking performed without recursion?
/// Each root rule creates one shared hidden stack at validation time ([VecDecue](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)), regardless of large nesting, the queue traverses its own stack without recursion
pub struct WasmRuleBytes(RuleBytes);

#[wasm_bindgen(js_class = "RuleBytes")]
impl WasmRuleBytes {
    #[wasm_bindgen(constructor)]
    /// Constructs a new `RuleBytes`
    /// # Arguments
    /// * `pattern` - a regular expression that will be used to search for matches
    /// * `requirement` - the requirement for the match
    /// # Notes
    /// * **by default, all rules must pass every match check**
    /// In this mode, to which all additional rules apply (default mode for everyone).
    /// We check that for each match (text) all the rules will work.
    /// ## Operation scheme of the mode
    /// ```bash
    /// #=======================================
    /// text = "txt [123] txt [456] txt [789]"
    /// #=======================================
    /// CustomError
    /// |
    /// |__ Rule "\[[^\[\]]+\]" (MustBeFound)
    ///      |   [123], [456], [789]
    ///      |___ Subrule ".+" (MustBeFound) ---> [123] -> [456] -> [789] -- TRUE
    ///      |                                      |       |        |
    ///      |___ Subrule "\[\d+\]" (MustBeFound) __|_______|________|
    ///
    /// ```
    /// * To load a `RuleBytes` into a `CartridgeBytes`, always use `finish_build()` at the end, after applying modifiers or initializing the rule to prepare the value for processing in `Rust`
    /// * Remember any modifier takes the contents of the `RuleBytes` body
    /// and returns a new one with a changed parameter (only `None` from the original Rule remains),
    /// so specify the modifier in the same place where you initialize `RuleBytes`.
    /// * Use `&[u8]` when searching for regex matches in haystacks. ([**FancyRegex**](https://crates.io/crates/fancy-regex) capabilities are not available)
    /// * Unicode support can be disabled, even if disabling it will result in a match with invalid `UTF-8` bytes. More info at [link](https://docs.rs/regex/latest/regex/bytes/index.html)
    /// * How is recursive structure checking performed without recursion?
    /// Each root rule creates one shared hidden stack at validation time ([VecDecue](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)), regardless of large nesting, the queue traverses its own stack without recursion
    pub fn new(pattern: String, requirement: WasmMatchRequirement) -> Self {
        console_error_panic_hook::set_once();
        Self(RuleBytes::new(pattern, requirement.into()))
    }
    /// Preparing value for processing in `Rust`
    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

impl From<WasmRuleBytes> for RuleBytes {
    fn from(value: WasmRuleBytes) -> Self {
        value.0
    }
}
