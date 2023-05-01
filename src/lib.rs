// Модуль отвечает за конвертацию `Python Type` в `Rust Type` и обратно
mod convert;
// Модуль отвеччает за создание кастомных ошибок и после проброс их же в `Python`
mod pyst_errors;
// Модуль отвечает за регулярные выражения, являются сердцем валидатора
mod regex_init;
// Crate py03 позволяет работать `Rust` вместе с `Python`
use pyo3::exceptions::{self};
use pyo3::prelude::*;
use pyo3::types::{PyList, PyType};
use std::str;

/// Класс содержит List ошибок по которым будет проходить обработка (re-export -> Python)
#[pyclass]
struct Validator {
    factory_data: Vec<(PyObject, Vec<String>)>,
}

/// Модуль для инициализации `Validator`
mod init_validator {
    use super::*;
    #[pymethods]
    impl Validator {
        /// Констуктор (re-export -> Python)
        #[new]
        pub fn py_new(factory_data: PyObject) -> PyResult<Validator> {
            // Используем привязку `GIL` для безопасной обраотки переменных из python
            // downcast служит проверкой типа объекта Python
            Python::with_gil(|py| match factory_data.downcast::<PyList>(py) {
                Ok(list_error_class) => Ok({
                    let mut factory_data: Vec<(Py<PyAny>, Vec<String>)> = Vec::new();
                    for item in convert::py_list_to_py_types(list_error_class)? {
                        factory_data
                            .push((item.to_object(py), regex_init::get_extra(item, "template")?));
                    }
                    // dbg!(&factory_data);
                    Validator { factory_data }
                }),
                Err(_) => Err(PyErr::new::<exceptions::PyTypeError, _>(
                    "'factoryData' must be a dictionary",
                )),
            })
        }
        pub fn validate(&self, raw_data: &[u8]) -> PyResult<()> {
            // проверка на конветрацию из byte в String (UTF8)
            match str::from_utf8(raw_data) {
                Ok(text) => {
                    // Используем привязку `GIL` для безопасной обраотки переменных из python
                    Python::with_gil(|py| {
                        for (class, extra) in &self.factory_data {
                            // convert::py_any_to_py_type(py, class);
                            // println!("{:#?}", );
                            // println!("{:#?} | {:#?}", class, extra);
                        }
                    })
                }
                Err(_) => {
                    return Err(PyErr::new::<exceptions::PyTypeError, _>(
                        "'factoryData' must be a dictionary",
                    ))
                }
            };
            Ok(())
        }
        /// минимальный набор методов для отладки
        fn __repr__(&self) -> PyResult<String> {
            Ok(format!("Validator({:#?})", self.factory_data))
        }

        fn __str__(&self) -> String {
            format!("{:#?}", self.factory_data)
        }
    }
}
// Модуль export необходим для импортирования : функций, методов, классов в `Python`
mod export {
    use super::*;
    /// отвечает за re-export, классов, методов, функций
    #[pymodule]
    fn pystval(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
        m.add_class::<Validator>()?;
        // m.add_function(wrap_pyfunction!(init_validator::validate, m)?)?;
        Ok(())
    }
}
