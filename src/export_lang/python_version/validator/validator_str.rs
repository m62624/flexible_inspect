use super::*;

#[pyclass(name = "TemplateValidator")]
pub struct PyTemplateValidator(TemplateValidator<Vec<Cartridge<Rule>>, Arc<str>>);

impl PyTemplateValidator {
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        todo!()
    }
}


