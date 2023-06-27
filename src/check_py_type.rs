use pyo3::{exceptions, types};

use super::*;

/// Исползьуется для проверки типов в конструкторе
pub fn pylist_from_pyobject<'a>(py: Python<'a>, obj: &'a PyObject) -> PyResult<&'a types::PyList> {
    if let Ok(list) = obj.downcast::<types::PyList>(py) {
        return Ok(list);
    }
    return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
        "`{}` -- Expected `List` --> List[Rule, Rule, Rule]",
        obj.as_ref(py).get_type().name().unwrap()
    )));
}

pub fn rule_from_pyobject(slf: &rule::Rule, packed_rule: &PyAny) -> PyResult<rule::Rule> {
    if let Ok(rule) = packed_rule.extract::<rule::Rule>() {
        return Ok(rule);
    }
    return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
        "Expected `Rule` in the list, the child error `{}` from the parent rule `{}`",
        packed_rule.get_type().name().unwrap(),
        slf.get_str_raw()?.get_str()
    )));
}
