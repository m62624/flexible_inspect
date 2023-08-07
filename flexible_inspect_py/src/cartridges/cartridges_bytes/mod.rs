mod modifiers;
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

impl From<PyCartridgeBytes> for Cartridge<RuleBytes> {
    fn from(value: PyCartridgeBytes) -> Self {
        value.0
    }
}