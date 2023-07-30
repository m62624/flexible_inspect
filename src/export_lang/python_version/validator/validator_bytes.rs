use super::*;

#[pyclass(name = "TemplateValidatorBytes")]
pub struct PyTemplateValidatorBytes(TemplateValidator<Vec<Cartridge<RuleBytes>>, Arc<[u8]>>);

impl PyTemplateValidatorBytes {
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        Ok(Self(TemplateValidator::new(
            PyTemplateValidatorBytes::_to_rust_for_new::<PyCartridgeBytes>(
                py,
                cartridges,
                "CartridgeBytes",
            )?,
        )))
    }
}

impl PyTemplateValidatorBase for PyTemplateValidatorBytes {
    type CartridgeTypeRust = Cartridge<RuleBytes>;
}
