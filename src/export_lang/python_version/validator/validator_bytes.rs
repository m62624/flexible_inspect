use super::*;

#[pyclass(name = "TemplateValidatorBytes")]
pub struct PyTemplateValidatorBytes(TemplateValidator<Vec<Cartridge<RuleBytes>>, Arc<[u8]>>);

