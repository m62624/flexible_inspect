use super::{
    BASE_ERROR, EXTRA_FROM_CLASS_PY, MESSAGE_WITH_EXTRA_FROM_CLASS_PY, MODULE_NAME,
    RULES_FROM_CLASS_PY,
};
use pyo3::AsPyPointer;
use pyo3::{prelude::*, types, PyNativeType};
use std::collections::HashMap;
pub fn init_base_error(_py: Python) -> Py<PyAny> {
    let dict = types::PyDict::new(_py);
    dict.set_item(MESSAGE_WITH_EXTRA_FROM_CLASS_PY, String::default())
        .unwrap();
    dict.set_item(EXTRA_FROM_CLASS_PY, HashMap::<String, String>::default())
        .unwrap();
    dict.set_item(RULES_FROM_CLASS_PY, HashMap::<String, usize>::default())
        .unwrap();

    PyErr::new_type(
        _py,
        &format!("{}.{}", MODULE_NAME, BASE_ERROR),
        None,
        Some(_py.get_type::<pyo3::exceptions::PyException>()),
        Some(dict.to_object(_py)),
    )
    .unwrap()
    .to_object(_py)
}

#[pyclass]
#[derive(Debug, Clone)]
/// Структура которая будет являться шаблоном класса в `python` для создание собвственных ошибок
pub struct PystvalError {
    message: String,
    extra: HashMap<String, String>,
    rules: HashMap<String, usize>,
}

#[pymethods]
impl PystvalError {
    #[new]
    #[pyo3(signature = (message, extra, rules, /))]
    pub fn new(
        _py: Python<'_>,
        message: String,
        extra: HashMap<String, String>,
        rules: HashMap<String, usize>,
    ) -> Self {
        Self {
            message,
            extra,
            rules,
        }
    }
}
impl ToPyObject for PystvalError {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let dict = types::PyDict::new(py);
        let extra = types::PyDict::new(py);
        let rules = types::PyDict::new(py);
        for (key, value) in self.extra.iter() {
            extra.set_item(key, value).unwrap();
        }
        for (key, value) in self.rules.iter() {
            rules.set_item(key, value).unwrap();
        }
        dict.setattr(MESSAGE_WITH_EXTRA_FROM_CLASS_PY, &self.message)
            .unwrap();
        dict.setattr(EXTRA_FROM_CLASS_PY, extra).unwrap();
        dict.setattr(RULES_FROM_CLASS_PY, rules).unwrap();
        dict.to_object(py)
    }
}

impl std::convert::From<&PystvalError> for pyo3::PyErr {
    #[inline]
    fn from(err: &PystvalError) -> pyo3::PyErr {
        Python::with_gil(|py| pyo3::PyErr::from_value(err.to_object(py).as_ref(py)))
    }
}

impl PystvalError {
    /// Creates a new [`PyErr`] of this type.
    ///
    /// [`PyErr`]: https://docs.rs/pyo3/latest/pyo3/struct.PyErr.html "PyErr in pyo3"
    #[inline]
    pub fn new_err<A>(args: A) -> pyo3::PyErr
    where
        A: pyo3::PyErrArguments + ::std::marker::Send + ::std::marker::Sync + 'static,
    {
        pyo3::PyErr::new::<PystvalError, A>(args)
    }
}

impl std::fmt::Display for PystvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut extra = String::new();
        let mut rules = String::new();
        for (key, value) in self.extra.iter() {
            extra.push_str(&format!("{}: {}\n", key, value));
        }
        for (key, value) in self.rules.iter() {
            rules.push_str(&format!("{}: {}\n", key, value));
        }
        write!(
            f,
            "message: {}\nextra:\n{}\nrules:\n{}",
            self.message, extra, rules
        )
    }
}
impl std::error::Error for PystvalError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        unsafe {
            let cause: &pyo3::exceptions::PyBaseException = self
                .py()
                .from_owned_ptr_or_opt(pyo3::ffi::PyException_GetCause(self.as_ptr()))?;

            ::std::option::Option::Some(cause)
        }
    }
}
unsafe impl pyo3::PyNativeType for PystvalError {
    fn py(&self) -> Python<'_> {
        unsafe { Python::assume_gil_acquired() }
    }

    unsafe fn unchecked_downcast(obj: &PyAny) -> &Self {
        &*(obj.as_ptr() as *const Self)
    }
}
impl pyo3::AsPyPointer for PystvalError {
    fn as_ptr(&self) -> *mut pyo3::ffi::PyObject {
        // self as *const Self as *mut _
        Python::with_gil(|py| PystvalError::to_object(self, py).as_ptr())
    }
}
