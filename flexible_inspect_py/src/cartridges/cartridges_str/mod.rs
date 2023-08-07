mod modifiers;
use super::*;

#[pyclass(name = "Cartridge")]
#[derive(Clone, Default)]
pub struct PyCartridge(Cartridge<Rule>);

#[pymethods]
impl PyCartridge {
    #[new]
    pub fn new(id: i32, message: String, root_rules: Vec<PyRule>) -> Self {
        Self(Cartridge::new(
            id,
            message,
            root_rules.into_iter().map(|rule| rule.into()),
        ))
    }
}

impl From<PyCartridge> for Cartridge<Rule> {
    fn from(value: PyCartridge) -> Self {
        value.0
    }
}
