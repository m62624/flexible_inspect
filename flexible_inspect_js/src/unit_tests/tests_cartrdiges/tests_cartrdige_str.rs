use super::*;

#[wasm_bindgen_test]
fn test_new_t_0() -> Result<(), JsValue> {
    let cartridge: Cartridge<Rule> = WasmCartridge::new(
        0,
        "message_0".into(),
        serde_wasm_bindgen::to_value(&vec![WasmRule::new(
            String::from(r"\w+"),
            WasmMatchRequirement::MustBeFound,
        )])?,
    )?
    .try_into()?;

    Ok(assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![Rule::new(r"\w+", MatchRequirement::MustBeFound)],
        ),
    ))
}

#[wasm_bindgen_test]
fn test_mode_match_t_0() -> Result<(), JsValue> {
    let cartridge: Cartridge<Rule> = WasmCartridge::new(
        0,
        "message_0".into(),
        serde_wasm_bindgen::to_value(&vec![WasmRule::new(
            String::from(r"\w+"),
            WasmMatchRequirement::MustBeFound,
        )])?,
    )?
    .any_r_for_any_m()?
    .try_into()?;

    Ok(assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![Rule::new(r"\w+", MatchRequirement::MustBeFound)],
        )
        .any_r_for_any_m(),
    ))
}
