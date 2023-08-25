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
        mem_self.0 = mem_self.0.map(|rule| rule.any_r_for_any_m());
        Ok(mem_self)
    }
}

impl TryFrom<PyCartridge> for Cartridge<Rule> {
    type Error = PyErr;

    fn try_from(value: PyCartridge) -> Result<Self, Self::Error> {
        value
            .0
            .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION_CARTRIDGE))
    }
}

// In the validator, we always put the rules into cartridges and the cartridges themselves into template_validator.
// This means that after applying modifiers, we need to get the same structure, but with different data.
// But when exporting to other languages, there is no ownership check when using `self`. But most likely there is a check with `&mut self`.
// To make changes safe, we use `std::mem::take`.
// This approach allows us to temporarily take data from an object without compromising its integrity.
// We then return the modified data back to the object.
// Yes, if you double `std::mem::take` you will get `None`, but this way you can safely call `panic!`,
// with your own warning why it happened and what to do about it
// If you export to other languages, don't worry,
// this is simply a way to safely change the state of objects passed to the &mut self method.
// This ensures efficient data management and predictable behavior when working
// with the library in different programming languages.
impl TryFrom<&mut PyCartridge> for PyCartridge {
    type Error = PyErr;

    fn try_from(value: &mut PyCartridge) -> Result<Self, Self::Error> {
        if value.0.is_some() {
            Ok(std::mem::take(value))
        } else {
            Err(PyErr::new::<exceptions::PyUnboundLocalError, _>(
                ERR_OPTION_CARTRIDGE,
            ))
        }
    }
}
