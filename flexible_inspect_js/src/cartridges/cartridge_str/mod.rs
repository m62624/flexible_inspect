mod modifiers;
use super::*;

/// The cartridge is the container of the rules.
///
/// # Notes:
/// * Use a container for one object if possible. Imagine that one container is one specific error `NotFound`, `InvalidHeader`, `WrongCase`.
#[wasm_bindgen(js_name = "Cartridge")]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WasmCartridge(pub(crate) Option<Cartridge<Rule>>);

#[wasm_bindgen(js_class = "Cartridge")]
impl WasmCartridge {
    /// Constructs a new `CartridgeBytes`
    /// # Arguments:
    /// * `error_code` - error code
    /// * `message` - error message
    /// * `root_rules` - \[ `Rule`, `Rule`, `Rule` \] (collection)
    /// # Notes:
    /// **by default, `all_rules_for_all_matches`**
    /// In this mode, all rules must be tested for all matches
    ///
    /// ## Fill in messages
    /// * Each cartridge supports filling the message with unwanted data, when specifying a message,
    /// you can specify a variable in the message in the format : **`{variable}`**.
    /// After specifying an identical group name in any rule along with the *`MustNotBeFound`* modifier
    #[wasm_bindgen(constructor)]
    pub fn new(
        error_code: i32,
        message: String,
        root_rules: JsValue,
    ) -> Result<WasmCartridge, JsValue> {
        console_error_panic_hook::set_once();
        Ok(Self(Some(Cartridge::new(
            error_code,
            message,
            serde_wasm_bindgen::from_value::<Vec<Rule>>(root_rules)
                .map_err(|_| JsValue::from_str(ERR_OPTION_RULE))?,
        ))))
    }

    pub fn finish_build(&self) -> Result<JsValue, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

impl TryFrom<WasmCartridge> for Cartridge<Rule> {
    type Error = JsValue;

    fn try_from(value: WasmCartridge) -> Result<Self, Self::Error> {
        value
            .0
            .ok_or_else(|| JsValue::from_str(ERR_OPTION_CARTRIDGE))
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
impl TryFrom<&mut WasmCartridge> for WasmCartridge {
    type Error = JsValue;

    fn try_from(value: &mut WasmCartridge) -> Result<Self, Self::Error> {
        if value.0.is_some() {
            Ok(std::mem::take(value))
        } else {
            Err(JsValue::from_str(ERR_OPTION_CARTRIDGE))
        }
    }
}
