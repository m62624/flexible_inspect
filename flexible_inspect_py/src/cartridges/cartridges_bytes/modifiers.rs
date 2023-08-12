use super::*;

#[pymethods]
impl PyCartridgeBytes {
    pub fn any_r_for_any_m(&mut self) -> Self {
        self.0 = self.0.any_r_for_any_m();
        std::mem::take(self)
    }
}
