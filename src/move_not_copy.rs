use crate::global_const::*;
use crate::*;
/// Конвертируем `PythonSafeObject` В `PyObject` (по сути это просто python абстракция)
impl ToPyObject for PythonSafeObject {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let dict = types::PyDict::new(py);
        // Добавляем `original_class` в словарь
        dict.set_item(CLASS_FOR_ERROR_PY, &self.original_class)
            .unwrap();
        // Преобразуем вектор `extra` в список `PyList` и добавляем его в словарь
        let extra_list = types::PyList::new(py, &self.extra);
        dict.set_item(EXTRA_FOR_ERROR_PY, &extra_list).unwrap();
        // Преобразуем вектор `regex_collection` в список `PyList` и добавляем его в словарь
        let regex_list = types::PyList::new(py, &self.rgxs);
        dict.set_item(REGEX_FOR_CHECK_DATA, &regex_list).unwrap();
        dict.into()
    }
}
/// Конвертируем `PyObject` В `PythonSafeObject` (`rust` структура)
impl<'p> FromPyObject<'p> for PythonSafeObject {
    fn extract(obj: &'p pyo3::PyAny) -> PyResult<Self> {
        let dict = obj.downcast::<types::PyDict>()?;
        let extra = dict
            .get_item(EXTRA_FOR_ERROR_PY)
            .unwrap()
            .downcast::<types::PyList>()?
            .iter()
            .map(|item| item.extract::<String>())
            .collect::<PyResult<Vec<String>>>()?;
        let rgxs = dict
            .get_item(REGEX_FOR_CHECK_DATA)
            .unwrap()
            .downcast::<types::PyList>()?
            .iter()
            .map(|item| item.extract::<String>())
            .collect::<PyResult<Vec<String>>>()?;
        Python::with_gil(|py| {
            let original_class = dict.get_item(CLASS_FOR_ERROR_PY).unwrap().to_object(py);
            Ok(PythonSafeObject {
                original_class,
                extra,
                rgxs,
            })
        })
    }
}

/// FromPyObject реалзиован после добавления макроса #[pyclass]
/// Конвертируем `Validator` В `PyObject` (по сути это просто python абстракция)
impl ToPyObject for Validator {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        let inner: Vec<PyObject> = self.inner.iter().map(|obj| obj.to_object(py)).collect();
        let dict = types::PyDict::new(py);
        dict.set_item(DATA_FOR_RUST, inner).unwrap();
        dict.into()
    }
}
