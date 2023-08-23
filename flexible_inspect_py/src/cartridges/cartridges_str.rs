use super::*;

#[pyclass(name = "Cartridge")]
#[derive(Clone, Default)]
pub struct PyCartridge(Option<Cartridge<Rule>>);

#[pymethods]
impl PyCartridge {
    #[new]
    pub fn new(id: i32, message: String, root_rules: Vec<PyRule>) -> PyResult<Self> {
        Ok(Self(Some(Cartridge::new(
            id,
            message,
            root_rules
                .into_iter()
                .map(|rule| rule.try_into())
                .collect::<PyResult<Vec<Rule>>>()?,
        ))))
    }
}

#[pymethods]
impl PyCartridge {
    pub fn any_r_for_any_m(&mut self) -> PyResult<Self> {
        let mut mem_self: PyCartridge = self.try_into()?;
        mem_self.0 = Some(mem_self.0.expect(ERR_OPTION).any_r_for_any_m());
        Ok(mem_self)
    }
}

impl TryFrom<PyCartridge> for Cartridge<Rule> {
    type Error = PyErr;

    fn try_from(value: PyCartridge) -> Result<Self, Self::Error> {
        value
            .0
            .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))
    }
}

impl TryFrom<&mut PyCartridge> for PyCartridge {
    type Error = PyErr;

    fn try_from(value: &mut PyCartridge) -> Result<Self, Self::Error> {
        let value = std::mem::take(value);
        if value.0.is_some() {
            Ok(value)
        } else {
            Err(PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))
        }
    }
}
