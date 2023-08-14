mod modifiers;
use super::*;

/// A rule is the minimum unit of logic in a validator.
/// The rule supports two regular expression crates:
/// [**Regex**](https://crates.io/crates/regex) and [**FancyRegex**](https://crates.io/crates/fancy-regex).
/// Determines which type is used based on the syntax (for example, if *Lookahead* and *Lookbehind* references are used, this automatically defines as [**FancyRegex**](https://crates.io/crates/fancy-regex)).
///
/// The most important feature is that the rule is recursive (don't worry, recursion is not used here).
/// Each rule can have nested rules, and these nested rules can have their own nested rules, and so on.
/// Thus, when the root rule is triggered, all the results obtained are passed to the nested rules, so you can build complex structural rules to suit any taste
///
/// # Notes
/// * To load a `Rule` into a `Cartridge`, always use `finish_build()` at the end, after applying modifiers or initializing the rule to prepare the value for processing in `Rust`
/// * Remember any modifier takes the contents of the `Rule` body
/// and returns a new one with a changed parameter (only `None` from the original Rule remains),
/// so specify the modifier in the same place where you initialize `Rule`.
/// * If you stick with the [**Regex**](https://crates.io/crates/regex) library features, all root and nested rules go into [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html).
/// Many expressions can be accommodated in a regular expression without *Lookahead* and *Lookbehind* references.
/// But this is just a recommendation. If you need to use references, of course you can specify them.
/// Then these rules will not be included in [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html),
/// and if there are rules in [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html) they will be the first in the queue to be checked, and those that use [**FancyRegex**](https://crates.io/crates/fancy-regex) features will be at the end of the queue
/// * Basically use `Rule` instead of `RuleBytes` when working with text (not necessarily just text, it also includes `html` structures, code fragments from other languages, etc.) since it has support for [**Regex**](https://crates.io/crates/regex) and [**FancyRegex**](https://crates.io/crates/fancy-regex).
/// * How is recursive structure checking performed without recursion?
/// Each root rule creates one shared hidden stack at validation time ([VecDecue](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)), regardless of large nesting, the queue traverses its own stack without recursion
#[wasm_bindgen(js_name = "Rule")]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WasmRule(Rule);
#[wasm_bindgen(js_class = "Rule")]
impl WasmRule {
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
    /// * To load a `Rule` into a `Cartridge`, always use `finish_build()` at the end, after applying modifiers or initializing the rule to prepare the value for processing in `Rust`
    /// * Remember any modifier takes the contents of the `Rule` body
    /// and returns a new one with a changed parameter (only `None` from the original Rule remains),
    /// so specify the modifier in the same place where you initialize `Rule`.
    /// * If you stick with the [**Regex**](https://crates.io/crates/regex) library features, all root and nested rules go into [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html).
    /// Many expressions can be accommodated in a regular expression without *Lookahead* and *Lookbehind* references.
    /// But this is just a recommendation. If you need to use references, of course you can specify them.
    /// Then these rules will not be included in [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html),
    /// and if there are rules in [**RegexSet**](https://docs.rs/regex/latest/regex/struct.RegexSet.html) they will be the first in the queue to be checked, and those that use [**FancyRegex**](https://crates.io/crates/fancy-regex) features will be at the end of the queue
    /// * Basically use `Rule` instead of `RuleBytes` when working with text (not necessarily just text, it also includes `html` structures, code fragments from other languages, etc.) since it has support for [**Regex**](https://crates.io/crates/regex) and [**FancyRegex**](https://crates.io/crates/fancy-regex).
    /// * How is recursive structure checking performed without recursion?
    /// Each root rule creates one shared hidden stack at validation time ([VecDecue](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)), regardless of large nesting, the queue traverses its own stack without recursion
    pub fn new(pattern: js_sys::RegExp, requirement: WasmMatchRequirement) -> Self {
        console_error_panic_hook::set_once();
        Self(Rule::new(pattern.source(), requirement.into()))
    }

    /// Preparing value for processing in `Rust`
    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

impl From<WasmRule> for Rule {
    fn from(value: WasmRule) -> Self {
        value.0
    }
}
