use super::*;

#[wasm_bindgen_test]
fn test_new_t_0() {
    let cartridge: Cartridge<RuleBytes> = WasmCartridgeBytes::new(
        0,
        "message_0".into(),
        serde_wasm_bindgen::to_value(&vec![WasmRuleBytes::new(
            js_sys::RegExp::new(r"\w+", ""),
            WasmMatchRequirement::MustBeFound,
        )])
        .unwrap(),
    )
    .unwrap()
    .into();

    assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![RuleBytes::new(r"\w+", MatchRequirement::MustBeFound)],
        ),
    )
}

#[wasm_bindgen_test]
fn test_mode_match_t_0() {
    let cartridge: Cartridge<RuleBytes> = WasmCartridgeBytes::new(
        0,
        "message_0".into(),
        serde_wasm_bindgen::to_value(&vec![WasmRuleBytes::new(
            js_sys::RegExp::new(r"\w+", ""),
            WasmMatchRequirement::MustBeFound,
        )])
        .unwrap(),
    )
    .unwrap()
    .any_r_for_any_m()
    .into();

    assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![RuleBytes::new(r"\w+", MatchRequirement::MustBeFound)],
        )
        .any_r_for_any_m(),
    )
}
