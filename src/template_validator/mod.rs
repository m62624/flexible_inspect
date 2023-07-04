use super::cartridge::CartridgeWrapper;
use super::custom_error;
use super::rule::next::NextStep;
use super::*;

mod validate;

#[derive(Debug)]
pub struct TemplateSafeSelf {
    cartridges: Vec<CartridgeWrapper>,
}

#[pyclass]
#[derive(Debug)]
pub struct TemplateValidator(Arc<TemplateSafeSelf>);

#[pymethods]
impl TemplateValidator {
    #[new]
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        Ok(Self(Arc::new(TemplateSafeSelf::new(py, cartridges)?)))
    }
}
