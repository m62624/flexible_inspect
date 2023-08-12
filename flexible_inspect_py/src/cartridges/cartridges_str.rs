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

#[pymethods]
impl PyCartridge {
    pub fn any_r_for_any_m(&mut self) -> Self {
        self.0 = self.0.any_r_for_any_m();
        std::mem::take(self)
    }
}

impl From<PyCartridge> for Cartridge<Rule> {
    fn from(value: PyCartridge) -> Self {
        value.0
    }
}
