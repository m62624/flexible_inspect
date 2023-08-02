use super::*;

mod fn_new {
    use super::*;
    #[wasm_bindgen_test]
    fn fn_new_t_0() {
        WasmRule::new(r"\d+".into(), MatchRequirement::MustBeFound);
    }

    #[wasm_bindgen_test]
    fn fn_extend_t_1() {
        WasmRule::new(r"\d+".into(), MatchRequirement::MustNotBeFound);
    }
}
