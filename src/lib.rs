mod export_py;
mod extra_init;
mod regex_init;
use async_mutex::Mutex;
use pyo3::prelude::*;
use pyo3::types;
use regex::Regex;
use std::sync::Arc;
//==============================================
// хранит само сообщение и **extra переменные
const MESSAGE_EXTRA: &'static str = "template";
// хранит регулярные выражения, является правилами обнаружения ошибок
const REGEGX_RULES: &'static str = "rules";
//==============================================
#[derive(Debug, Clone)]
pub struct PythonSafeObject {
    original_class: PyObject,
    extra: Vec<String>,
    regex_collection: Vec<Regex>,
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Validator {
    inner: Arc<Mutex<Vec<PythonSafeObject>>>,
}
mod init_validator {
    use super::*;
    #[pymethods]
    impl Validator {
        #[new]
        fn py_constructor(errors_list: PyObject) -> PyResult<Self> {
            Python::with_gil(|py| {
                let mut class_templates: Vec<PythonSafeObject> = Vec::new();
                for boxed_class in errors_list.downcast::<types::PyList>(py)?.iter() {
                    let unboxed_class = boxed_class.downcast::<types::PyType>()?;
                    class_templates.push(PythonSafeObject {
                        original_class: unboxed_class.to_object(py),
                        extra: extra_init::extra_from_class(unboxed_class, MESSAGE_EXTRA)?,
                        regex_collection: regex_init::regex_from_class(unboxed_class)?,
                    });
                }
                let temp = Validator {
                    inner: Arc::new(Mutex::new(class_templates)),
                };
                dbg!(&temp);
                Ok(temp)
            })
        }
        /// минимальный набор методов для отладки
        fn __repr__(&self) -> PyResult<String> {
            Ok(format!("Validator({:#?})", self.inner))
        }
        fn __str__(&self) -> String {
            format!("{:#?}", self.inner)
        }
    }
}
