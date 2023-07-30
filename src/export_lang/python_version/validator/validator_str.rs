use super::*;

#[pyclass(name = "TemplateValidator")]
pub struct PyTemplateValidator(TemplateValidator<Vec<Cartridge<Rule>>, Arc<str>>);

impl PyTemplateValidator {
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        Ok(Self(TemplateValidator::new(
            PyTemplateValidator::_to_rust_for_new::<PyCartridge>(py, cartridges, "Cartridge")?,
        )))
    }
}

impl PyTemplateValidatorBase for PyTemplateValidator {
    type CartridgeTypeRust = Cartridge<Rule>;
}
