use super::*;
pub mod actions_from_the_requirement;
use super::cartridge::CartridgeWrapper;
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

impl TemplateSafeSelf {
    fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        if let Ok(list) = cartridges.downcast::<types::PyList>(py) {
            list.iter()
                .map(|class_obj| {
                    if let Ok(class_py) = class_obj.downcast::<types::PyType>() {
                        CartridgeWrapper::new(py, class_py.to_object(py))?;
                        Ok(())
                    } else {
                        return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                            "'{}' must be a 'Class'",
                            class_obj
                        )));
                    }
                })
                .collect::<PyResult<()>>()?;
        }
        return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "'{}' must be a 'List[ Class, Class... ]'",
            cartridges
        )));
    }
}
