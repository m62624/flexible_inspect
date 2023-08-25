mod modifiers;
use super::*;

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
#[wasm_bindgen(js_name = "RuleBytes")]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WasmRuleBytes(pub(crate) Option<RuleBytes>);

#[wasm_bindgen(js_class = "RuleBytes")]
impl WasmRuleBytes {
    /// Constructs a new `RuleBytes`
    /// # Arguments
    /// * `pattern` - a regular expression that will be used to search for matches
    /// * `requirement` - the requirement for the match
    /// # Notes
    /// * By default, `all_rules_for_all_matches`
    /// * To load a `RuleBytes` into a `CartridgeBytes`, always use `finish_build()` at the end, after applying modifiers or initializing the rule to prepare the value for processing in `Rust`
    /// * Remember any modifier takes the contents of the `RuleBytes` body
    /// and returns a new one with a changed parameter (only `None` from the original Rule remains),
    /// so specify the modifier in the same place where you initialize `RuleBytes`.
    /// * Use `&[u8]` when searching for regex matches in haystacks. ([**FancyRegex**](https://crates.io/crates/fancy-regex) capabilities are not available)
    /// * Unicode support can be disabled, even if disabling it will result in a match with invalid `UTF-8` bytes. More info at [link](https://docs.rs/regex/latest/regex/bytes/index.html)
    /// * How is recursive structure checking performed without recursion?
    /// Each root rule creates one shared hidden stack at validation time ([VecDecue](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)), regardless of large nesting, the queue traverses its own stack without recursion
    #[wasm_bindgen(constructor)]
    pub fn new(pattern: String, requirement: WasmMatchRequirement) -> Self {
        console_error_panic_hook::set_once();
        Self(Some(RuleBytes::new(pattern, requirement.into())))
    }
    /// Preparing value for processing in `Rust`
    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

// In the validator, we always put the rules into cartridges and the cartridges themselves into template_validator.
// This means that after applying modifiers, we need to get the same structure, but with different data.
// But when exporting to other languages, there is no ownership check when using `self`. But most likely there is a check with `&mut self`.
// To make changes safe, we use `std::mem::take`.
// This approach allows us to temporarily take data from an object without compromising its integrity.
// We then return the modified data back to the object.
// Yes, if you double `std::mem::take` you will get `None`, but this way you can safely call `panic!`,
// with your own warning why it happened and what to do about it
// If you export to other languages, don't worry,
// this is simply a way to safely change the state of objects passed to the &mut self method.
// This ensures efficient data management and predictable behavior when working
// with the library in different programming languages.
impl TryFrom<&mut WasmRuleBytes> for WasmRuleBytes {
    type Error = JsValue;

    fn try_from(value: &mut WasmRuleBytes) -> Result<Self, Self::Error> {
        if value.0.is_some() {
            Ok(std::mem::take(value))
        } else {
            Err(JsValue::from_str(ERR_OPTION_RULE_BYTES))
        }
    }
}

impl TryFrom<WasmRuleBytes> for RuleBytes {
    type Error = JsValue;

    fn try_from(value: WasmRuleBytes) -> Result<Self, Self::Error> {
        value
            .0
            .ok_or_else(|| JsValue::from_str(ERR_OPTION_RULE_BYTES))
    }
}
