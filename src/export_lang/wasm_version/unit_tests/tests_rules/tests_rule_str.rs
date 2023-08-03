use super::*;

pub mod fn_start_build {
    use super::*;
    #[wasm_bindgen_test]
    fn fn_start_build_t_0() {
        assert_ne!(
            WasmRule::start_build(r"\d+".into(), MatchRequirement::MustBeFound),
            WasmRule::start_build(r"\s+".into(), MatchRequirement::MustBeFound)
        );
    }

    #[wasm_bindgen_test]
    fn fn_start_build_t_1() {
        assert_eq!(
            WasmRule::start_build(r"\d+".into(), MatchRequirement::MustBeFound),
            WasmRule::start_build(r"\d+".into(), MatchRequirement::MustBeFound)
        );
    }

    #[wasm_bindgen_test]
    fn fn_finish_build_t_0() {
        assert_eq!(
            serde_wasm_bindgen::from_value::<WasmRule>(
                WasmRule::start_build(r"\d+".into(), MatchRequirement::MustBeFound)
                    .finish_build()
                    .unwrap()
            )
            .unwrap(),
            WasmRule::start_build(r"\d+".into(), MatchRequirement::MustBeFound)
        );
    }

    #[wasm_bindgen_test]
    fn fn_finish_build_t_1() {
        assert_ne!(
            serde_wasm_bindgen::from_value::<WasmRule>(
                WasmRule::start_build(r"\d+".into(), MatchRequirement::MustBeFound)
                    .finish_build()
                    .unwrap()
            )
            .unwrap(),
            WasmRule::start_build(r"\r+".into(), MatchRequirement::MustBeFound)
        );
    }
}
