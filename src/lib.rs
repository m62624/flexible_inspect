mod convert;
mod pyst_errors;
use pyo3::{exceptions, prelude::*, types::PyDict};

#[pyclass]
struct Validator {
    factory_data: PyObject,
}

mod init_validator {
    use super::*;
    #[pymethods]
    impl Validator {
        #[new]
        fn py_new(factory_data: PyObject) -> PyResult<Validator> {
            Python::with_gil(|py| match factory_data.downcast::<PyDict>(py) {
                Ok(value) => Ok({
                    convert::get_hashmap_from_dict(value)?;
                    // BaseError::py_show_attr(py, value);
                    Validator { factory_data }
                }),
                Err(_) => Err(PyErr::new::<exceptions::PyTypeError, _>(
                    "factory_data must be a dictionary",
                )),
            })
        }
        fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
            Ok(format!(
                "{}({})",
                slf.get_type().name()?,
                slf.borrow().factory_data
            ))
        }

        fn __str__(&self) -> String {
            self.factory_data.to_string()
        }
    }
}
// Модуль export необходим для импортирования : функций, методов, классов в `Python`
mod export {
    use super::*;
    #[pymodule]
    fn pystval(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
        m.add_class::<Validator>()?;
        m.add_function(wrap_pyfunction!(pyst_errors::throw_error, m)?)?;
        Ok(())
    }
}
