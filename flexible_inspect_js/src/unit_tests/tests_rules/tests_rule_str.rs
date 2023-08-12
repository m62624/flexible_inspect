use super::*;

#[wasm_bindgen_test]
pub fn test_new_t_0() {
    let rule: Rule = WasmRule::new(
        js_sys::RegExp::new(r"\w+", ""),
        WasmMatchRequirement::MustBeFound,
    )
    .into();

    assert_eq!(rule, Rule::new(r"\w+", MatchRequirement::MustBeFound));
}

#[wasm_bindgen_test]
pub fn test_new_t_1() {
    let rule: Rule = WasmRule::new(
        js_sys::RegExp::new(r"\w+", ""),
        WasmMatchRequirement::MustNotBeFound,
    )
    .into();

    assert_eq!(rule, Rule::new(r"\w+", MatchRequirement::MustNotBeFound));
}

#[wasm_bindgen_test]
fn test_extend_t_0() {
    let rule: Rule = WasmRule::new(
        js_sys::RegExp::new(r"\w+", ""),
        WasmMatchRequirement::MustNotBeFound,
    )
    .extend(
        serde_wasm_bindgen::to_value::<Vec<WasmRule>>(&vec![WasmRule::new(
            js_sys::RegExp::new(r"\w+", ""),
            WasmMatchRequirement::MustNotBeFound,
        )])
        .unwrap(),
    )
    .unwrap()
    .into();

    assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound)
            .extend(vec![Rule::new(r"\w+", MatchRequirement::MustNotBeFound),])
    );
}
