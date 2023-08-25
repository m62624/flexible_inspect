use super::*;

#[wasm_bindgen(js_class=Cartridge)]
impl WasmCartridge {
    /// modifier to change the rule matching mode.
    ///
    /// In this mode, at least one rule must pass at least one match check
    pub fn any_r_for_any_m(&mut self) -> Result<WasmCartridge, JsValue> {
        let mut mem_self: WasmCartridge = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.any_r_for_any_m());
        Ok(mem_self)
    }
}
