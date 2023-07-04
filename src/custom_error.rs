use super::*;

pub fn make_error(
    py: Python,
    custom_class_error: &PyObject,
    extra_with_value: Option<HashMap<String, String>>,
) -> PyResult<()> {
    // Создаем объект класса ошибки с переданными параметрами
    let extra = types::PyDict::new(py);
    if let Some(extra_hm) = extra_with_value {
        for (key, value) in extra_hm {
            extra.set_item(key, value)?;
        }
    }
    Err(PyErr::new::<PyException, _>(
        custom_class_error
            .downcast::<PyAny>(py)?
            .call(types::PyTuple::empty(py), Some(extra))?
            .to_object(py),
    )
    .into())
}
