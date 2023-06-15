use pyo3::{create_exception, exceptions::PyException, prelude::*};
use std::collections::HashMap;

#[pyclass]
#[derive(Debug)]
/// Структура которая будет являться шаблоном класса в `python` для создание собственных ошибок
pub struct BaseError {
    message: String,
    extra: HashMap<String, String>,
    rules: HashMap<String, usize>,
}
#[pymethods]
impl BaseError {
    #[new]
    #[pyo3(signature = (message = None, extra = None, rules = None))]
    pub fn new(
        message: Option<String>,
        extra: Option<HashMap<String, String>>,
        rules: Option<HashMap<String, usize>>,
    ) -> Self {
        BaseError {
            message: message.unwrap_or_default(),
            extra: extra.unwrap_or_default(),
            rules: rules.unwrap_or_default(),
        }
    }

    #[getter]
    pub fn message(&self) -> PyResult<String> {
        Ok(self.message.clone())
    }

    #[getter]
    pub fn extra(&self) -> PyResult<HashMap<String, String>> {
        Ok(self.extra.clone())
    }

    #[getter]
    pub fn rules(&self) -> PyResult<HashMap<String, usize>> {
        Ok(self.rules.clone())
    }
}

// ======================== Импорт шаблонной ошибки в Python ===========================
// create_exception!(pystval,BaseError,PyException);
// fn temp_plug() {
//     Python::with_gil(|py| {
//         PyErr::new_type(
//             py,
//             "pystval",
//             None,
//             Some(py.get_type::<PyException>()),
//             dict,
//         );
//     });
// }
