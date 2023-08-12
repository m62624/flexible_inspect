use super::*;

#[pyclass(name = "CartridgeBytes")]
#[derive(Clone, Default)]
pub struct PyCartridgeBytes(Cartridge<RuleBytes>);

#[pymethods]
impl PyCartridgeBytes {
    #[new]
    pub fn new(id: i32, message: String, root_rules: Vec<PyRuleBytes>) -> Self {
        Self(Cartridge::new(
            id,
            message,
            root_rules.into_iter().map(|rule| rule.into()),
        ))
    }
}

#[pymethods]
impl PyCartridgeBytes {
    pub fn any_r_for_any_m(&mut self) -> Self {
        self.0 = self.0.any_r_for_any_m();
        std::mem::take(self)
    }
}

impl From<PyCartridgeBytes> for Cartridge<RuleBytes> {
    fn from(value: PyCartridgeBytes) -> Self {
        value.0
    }
}
