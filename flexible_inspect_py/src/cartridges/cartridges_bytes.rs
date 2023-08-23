use super::*;

#[pyclass(name = "CartridgeBytes")]
#[derive(Clone, Default)]
pub struct PyCartridgeBytes(Option<Cartridge<RuleBytes>>);

#[pymethods]
impl PyCartridgeBytes {
    #[new]
    pub fn new(id: i32, message: String, root_rules: Vec<PyRuleBytes>) -> PyResult<Self> {
        Ok(Self(Some(Cartridge::new(
            id,
            message,
            root_rules
                .into_iter()
                .map(|rule| rule.try_into())
                .collect::<PyResult<Vec<RuleBytes>>>()?,
        ))))
    }
}

#[pymethods]
impl PyCartridgeBytes {
    pub fn any_r_for_any_m(&mut self) -> PyResult<Self> {
        let mut mem_self: PyCartridgeBytes = self.try_into()?;
        mem_self.0 = Some(mem_self.0.expect(ERR_OPTION).any_r_for_any_m());
        Ok(mem_self)
    }
}

impl TryFrom<PyCartridgeBytes> for Cartridge<RuleBytes> {
    type Error = PyErr;

    fn try_from(value: PyCartridgeBytes) -> Result<Self, Self::Error> {
        value
            .0
            .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))
    }
}

impl TryFrom<&mut PyCartridgeBytes> for PyCartridgeBytes {
    type Error = PyErr;

    fn try_from(value: &mut PyCartridgeBytes) -> Result<Self, Self::Error> {
        let value = std::mem::take(value);
        if value.0.is_some() {
            Ok(value)
        } else {
            Err(PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))
        }
    }
}
