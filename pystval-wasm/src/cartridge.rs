use std::sync::Arc;

use super::*;

enum CartridgeData {
    RuleData(Cartridge<Rule>),
    RuleBytesData(Cartridge<RuleBytes>),
}

#[wasm_bindgen(js_name = Cartridge)]
pub struct WasmCartridge {
    cartridge_data: CartridgeData,
}

#[wasm_bindgen(js_class=Cartridge)]
impl WasmCartridge {
    #[wasm_bindgen(constructor)]
    pub fn new(error_code: i32, message: String, root_rules: JsValue) -> Self {
        let root_rules = Arc::new(root_rules);
        let root_rules_str = Arc::clone(&root_rules);
        let root_rulles_bytes = Arc::clone(&root_rules);
        if let Ok(value) = serde_wasm_bindgen::from_value::<Vec<Rule>>(
            Arc::downgrade(&root_rules_str).as_ptr().into(),
        ) {
            return WasmCartridge {
                cartridge_data: CartridgeData::RuleData(Cartridge::new(error_code, message, value)),
            };
        } else if let Ok(value) = serde_wasm_bindgen::from_value::<Vec<RuleBytes>>(
            Arc::downgrade(&root_rulles_bytes).as_ptr().into(),
        ) {
            return WasmCartridge {
                cartridge_data: CartridgeData::RuleBytesData(Cartridge::new(
                    error_code, message, value,
                )),
            };
        } else {
            panic!("Invalid root rules");
        }
    }
}
