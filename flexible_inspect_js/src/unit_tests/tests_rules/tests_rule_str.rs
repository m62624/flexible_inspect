use super::*;

#[wasm_bindgen_test]
pub fn test_new_t_0() -> Result<(), JsValue> {
    let rule: Rule =
        WasmRule::new(String::from(r"\w+"), WasmMatchRequirement::MustBeFound).try_into()?;

    Ok(assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustBeFound)
    ))
}

#[wasm_bindgen_test]
pub fn test_new_t_1() -> Result<(), JsValue> {
    let rule: Rule =
        WasmRule::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound).try_into()?;

    Ok(assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound)
    ))
}

#[wasm_bindgen_test]
fn test_extend_t_0() -> Result<(), JsValue> {
    let rule: Rule = WasmRule::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
        .extend(serde_wasm_bindgen::to_value::<Vec<WasmRule>>(&vec![
            WasmRule::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound),
        ])?)?
        .try_into()?;

    Ok(assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound)
            .extend(vec![Rule::new(r"\w+", MatchRequirement::MustNotBeFound)])
    ))
}

#[wasm_bindgen_test]
fn test_mode_counter_t_0() -> Result<(), JsValue> {
    let rule: Rule = WasmRule::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
        .counter_is_equal(1)?
        .try_into()?;

    Ok(assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).counter_is_equal(1)
    ))
}

#[wasm_bindgen_test]
fn test_mode_counter_t_1() -> Result<(), JsValue> {
    let rule: Rule = WasmRule::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
        .counter_less_than(1)?
        .try_into()?;

    Ok(assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).counter_less_than(1)
    ))
}

#[wasm_bindgen_test]
fn test_mode_counter_t_2() -> Result<(), JsValue> {
    let rule: Rule = WasmRule::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
        .counter_more_than(1)?
        .try_into()?;

    Ok(assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).counter_more_than(1)
    ))
}

#[wasm_bindgen_test]
fn test_mode_match_t_0() -> Result<(), JsValue> {
    let rule: Rule = WasmRule::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
        .all_r_for_any_m()?
        .try_into()?;

    Ok(assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).all_r_for_any_m()
    ))
}

#[wasm_bindgen_test]
fn test_mode_match_t_1() -> Result<(), JsValue> {
    let rule: Rule = WasmRule::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
        .any_r_for_all_m()?
        .try_into()?;

    Ok(assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).any_r_for_all_m()
    ))
}

#[wasm_bindgen_test]
fn test_mode_match_t_2() -> Result<(), JsValue> {
    let rule: Rule = WasmRule::new(String::from(r"\w+"), WasmMatchRequirement::MustNotBeFound)
        .any_r_for_any_m()?
        .try_into()?;

    Ok(assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).any_r_for_any_m()
    ))
}
