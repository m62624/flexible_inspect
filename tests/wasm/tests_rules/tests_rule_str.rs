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
