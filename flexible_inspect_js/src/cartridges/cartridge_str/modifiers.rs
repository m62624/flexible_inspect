use super::*;

#[wasm_bindgen(js_class=Cartridge)]
impl WasmCartridge {
    pub fn any_r_for_any_m(&mut self) -> Self {
        self.0 = self.0.any_r_for_any_m();
        std::mem::take(self)
    }
}
