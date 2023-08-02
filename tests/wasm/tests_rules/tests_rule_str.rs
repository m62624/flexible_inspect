use super::*;

mod fn_new {
    use super::*;
    #[wasm_bindgen_test]
    fn fn_new_t_0() {
        WasmRuleBytes::new(r"\d+".into(), MatchRequirement::MustBeFound);
    }

    #[wasm_bindgen_test]
    fn fn_new_t_1() {
        WasmRuleBytes::new(r"\d+".into(), MatchRequirement::MustNotBeFound);
    }
}

mod fn_extend {

    use super::*;
    #[wasm_bindgen_test]
    fn fn_extend_t_0() {
        let nested_rules = vec![
            serde_wasm_bindgen::to_value(&WasmRuleBytes::new(
                r"\d+".into(),
                MatchRequirement::MustBeFound,
            ))
            .unwrap(),
            serde_wasm_bindgen::to_value(&WasmRuleBytes::new(
                r"\d+".into(),
                MatchRequirement::MustBeFound,
            ))
            .unwrap(),
        ];
        WasmRuleBytes::new(r"\d+".into(), MatchRequirement::MustBeFound)
            .extend(nested_rules)
            .unwrap();
    }
}
