use super::{
    BASE_ERROR, EXTRA_FROM_CLASS_PY, MESSAGE_WITH_EXTRA_FROM_CLASS_PY, MODULE_NAME,
    RULES_FROM_CLASS_PY,
};
use pyo3::{prelude::*, types};
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
