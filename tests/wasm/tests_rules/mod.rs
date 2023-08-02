use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1, 1);
}

#[wasm_bindgen_test]
#[should_panic]
fn fail() {
    assert_eq!(1, 2);
}
