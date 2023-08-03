use super::*;

#[wasm_bindgen_test]
fn fn_start_build_0() {
    assert_ne!(
        WasmRuleBytes::start_build(r"\d+".into(), MatchRequirement::MustBeFound),
        WasmRuleBytes::start_build(r"\s+".into(), MatchRequirement::MustBeFound)
    );
}

#[wasm_bindgen_test]
fn fn_start_build_1() {
    assert_eq!(
        WasmRuleBytes::start_build(r"\d+".into(), MatchRequirement::MustBeFound),
        WasmRuleBytes::start_build(r"\d+".into(), MatchRequirement::MustBeFound)
    );
}

#[wasm_bindgen_test]
fn fn_finish_build_0() {
    assert_eq!(
        serde_wasm_bindgen::from_value::<WasmRuleBytes>(
            WasmRuleBytes::start_build(r"\d+".into(), MatchRequirement::MustBeFound)
                .finish_build()
                .unwrap()
        )
        .unwrap(),
        WasmRuleBytes::start_build(r"\d+".into(), MatchRequirement::MustBeFound)
    );
}

#[wasm_bindgen_test]
fn fn_finish_build_1() {
    assert_ne!(
        serde_wasm_bindgen::from_value::<WasmRuleBytes>(
            WasmRuleBytes::start_build(r"\d+".into(), MatchRequirement::MustBeFound)
                .finish_build()
                .unwrap()
        )
        .unwrap(),
        WasmRuleBytes::start_build(r"\r+".into(), MatchRequirement::MustBeFound)
    );
}
