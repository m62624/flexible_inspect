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

impl TemplateSafeSelf {
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        let mut data = Vec::new();
        if let Ok(list) = cartridges.downcast::<types::PyList>(py) {
            list.iter().try_for_each(|class_obj| {
                if let Ok(class_py) = class_obj.downcast::<types::PyType>() {
                    data.push(CartridgeWrapper::new(py, class_py.to_object(py))?);
                    Ok(())
                } else {
                    Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "'{}' must be a 'Class'",
                        class_obj
                    )))
                }
            })?;
            Ok(Self { cartridges: data })
        } else {
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'List[ Class, Class... ]'",
                cartridges
            )));
        }
    }
}
