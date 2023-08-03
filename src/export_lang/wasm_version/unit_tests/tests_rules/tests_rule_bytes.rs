use super::*;

pub mod fn_start_build {
    use super::*;
    #[wasm_bindgen_test]
    fn fn_start_build_t_0() {
        assert_ne!(
            WasmRuleBytes::start_build(r"\d+".into(), MatchRequirement::MustBeFound),
            WasmRuleBytes::start_build(r"\s+".into(), MatchRequirement::MustBeFound)
        );
    }

    #[wasm_bindgen_test]
    fn fn_start_build_t_1() {
        assert_eq!(
            WasmRuleBytes::start_build(r"\d+".into(), MatchRequirement::MustBeFound),
            WasmRuleBytes::start_build(r"\d+".into(), MatchRequirement::MustBeFound)
        );
    }

    #[wasm_bindgen_test]
    fn fn_finish_build_t_0() {
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
    fn fn_finish_build_t_1() {
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
}

pub mod fn_extend {
    use super::*;

    #[wasm_bindgen_test]
    fn fn_extend_t_0() {
        let nested_rules = vec![
            serde_wasm_bindgen::to_value(&WasmRuleBytes::start_build(
                r"ABOBA".into(),
                MatchRequirement::MustBeFound,
            ))
            .unwrap(),
            serde_wasm_bindgen::to_value(&WasmRuleBytes::start_build(
                r"BOB".into(),
                MatchRequirement::MustBeFound,
            ))
            .unwrap(),
            serde_wasm_bindgen::to_value(&WasmRuleBytes::start_build(
                r"LOCK".into(),
                MatchRequirement::MustBeFound,
            ))
            .unwrap(),
            serde_wasm_bindgen::to_value(&WasmRuleBytes::start_build(
                r"UNLOCK".into(),
                MatchRequirement::MustBeFound,
            ))
            .unwrap(),
        ];

        WasmRuleBytes::start_build(r".+".into(), MatchRequirement::MustNotBeFound)
            .extend(nested_rules);
    }
}
