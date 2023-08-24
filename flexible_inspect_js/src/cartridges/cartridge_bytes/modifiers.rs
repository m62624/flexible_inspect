use super::*;

#[wasm_bindgen(js_class=CartridgeBytes)]
impl WasmCartridgeBytes {
    pub fn any_r_for_any_m(&mut self) -> Result<WasmCartridgeBytes, JsValue> {
        let mut mem_self: WasmCartridgeBytes = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.any_r_for_any_m());
        Ok(mem_self)
    }
}
