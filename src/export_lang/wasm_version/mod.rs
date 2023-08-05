pub mod base_error;
pub mod cartridges;
pub mod rules;
#[cfg(test)]
pub mod unit_tests;
pub mod validator;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

fn _to_rust_for_extend<U: for<'de> Deserialize<'de>>(
    nested_rules: Vec<JsValue>,
    message: &str,
    message_if_empty: &str,
) -> Result<Vec<U>, JsValue> {
    if nested_rules.is_empty() {
        return Err(JsValue::from_str(format!("{message_if_empty}").as_str()));
    }
    nested_rules
        .into_iter()
        .map(|rule_js| {
            serde_wasm_bindgen::from_value::<U>(rule_js)
                .map(|rule| Ok(rule))
                .unwrap_or_else(|_| Err(JsValue::from_str(format!("{message}").as_str())))
        })
        .collect::<Result<Vec<U>, JsValue>>()
}
