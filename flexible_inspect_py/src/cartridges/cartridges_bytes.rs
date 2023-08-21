use super::*;

#[pyclass(name = "CartridgeBytes")]
#[derive(Clone, Default)]
pub struct PyCartridgeBytes(Option<Cartridge<RuleBytes>>);

#[pymethods]
impl PyCartridgeBytes {
    #[new]
    pub fn new(id: i32, message: String, root_rules: Vec<PyRuleBytes>) -> Self {
        Self(Some(Cartridge::new(
            id,
            message,
            root_rules.into_iter().map(|rule| rule.into()),
        )))
    }
}

#[pymethods]
impl PyCartridgeBytes {
    pub fn any_r_for_any_m(&mut self) -> Self {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(m_self.0.expect(ERR_OPTION).any_r_for_any_m());
        m_self
    }
}

impl From<PyCartridgeBytes> for Cartridge<RuleBytes> {
    fn from(value: PyCartridgeBytes) -> Self {
        value.0.expect(ERR_OPTION)
    }
}
