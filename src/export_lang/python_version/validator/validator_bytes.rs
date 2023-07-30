use super::*;

#[pyclass(name = "TemplateValidatorBytes")]
pub struct PyTemplateValidatorBytes(TemplateValidator<Vec<PyCartridgeBytes>, Box<[u8]>>);
