use super::*;

#[pyclass(name = "Cartridge")]
#[derive(Clone, Default)]
pub struct PyCartridge(Option<Cartridge<Rule>>);

#[pymethods]
impl PyCartridge {
    #[new]
    pub fn new(id: i32, message: String, root_rules: Vec<PyRule>) -> Self {
        Self(Some(Cartridge::new(
            id,
            message,
            root_rules.into_iter().map(|rule| rule.into()),
        )))
    }
}

#[pymethods]
impl PyCartridge {
    pub fn any_r_for_any_m(&mut self) -> Self {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(m_self.0.expect(ERR_OPTION).any_r_for_any_m());
        m_self
    }
}

impl From<PyCartridge> for Cartridge<Rule> {
    fn from(value: PyCartridge) -> Self {
        value.0.expect(ERR_OPTION)
    }
}
