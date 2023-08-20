use super::*;

#[wasm_bindgen_test]
fn test_validate_t_0() {
    let cartrdige_1 = WasmCartridge::new(
        1,
        "message_1".into(),
        serde_wasm_bindgen::to_value(&vec![WasmRule::new(
            String::from(r"ABC"),
            WasmMatchRequirement::MustNotBeFound,
        )])
        .unwrap(),
    )
    .unwrap();
    let cartridge_2 = WasmCartridge::new(
        2,
        "message_2".into(),
        serde_wasm_bindgen::to_value(&vec![WasmRule::new(
            String::from(r"\d+"),
            WasmMatchRequirement::MustBeFound,
        )])
        .unwrap(),
    )
    .unwrap();
    let validator = WasmTemplateValidator::new(
        serde_wasm_bindgen::to_value(&vec![cartrdige_1, cartridge_2]).unwrap(),
    )
    .unwrap();
    assert!(validator.validate("123".to_string()).is_none());
    assert!(!validator.validate("ABC 123".to_string()).is_none());
    assert_eq!(validator.validate("ABC".to_string()).unwrap().0.len(), 2);
}

#[wasm_bindgen_test]
fn test_validate_t_1() {
    let cartrdige_1 = WasmCartridgeBytes::new(
        1,
        "message_1".into(),
        serde_wasm_bindgen::to_value(&vec![WasmRuleBytes::new(
            String::from(r"ABC"),
            WasmMatchRequirement::MustNotBeFound,
        )])
        .unwrap(),
    )
    .unwrap();
    let cartridge_2 = WasmCartridgeBytes::new(
        2,
        "message_2".into(),
        serde_wasm_bindgen::to_value(&vec![WasmRuleBytes::new(
            String::from(r"\d+"),
            WasmMatchRequirement::MustBeFound,
        )])
        .unwrap(),
    )
    .unwrap();
    let validator = WasmTemplateValidatorBytes::new(
        serde_wasm_bindgen::to_value(&vec![cartrdige_1, cartridge_2]).unwrap(),
    )
    .unwrap();
    assert!(validator.validate("123".as_bytes()).is_none());
    assert!(!validator.validate("ABC 123".as_bytes()).is_none());
    assert_eq!(validator.validate("ABC".as_bytes()).unwrap().0.len(), 2);
}
