mod export_py;
mod global_const;
mod init;
mod move_not_copy;
mod make_py_error;
use global_const::*;
use pyo3::prelude::*;
use pyo3::types;
use pyo3::ToPyObject;
use std::sync::Arc;
// Реалзиуем Clone, так как необходимо передавать `self` в `async fn`
#[derive(Debug, Clone)]
pub struct PythonSafeObject {
    // Храним переданный класс на основе которого создаем ошибку
    original_class: PyObject,
    // Храним **extra для заполнения информаций ошибки
    extra: Vec<String>,
    // Храним регулярные выражения для обработки текста
    rgxs: Vec<String>,
}
// Реалзиуем Clone, так как необходимо передавать `self` в `async fn`
#[derive(Debug, Clone)]
#[pyclass]
pub struct Validator {
    // Храним коллекцию, где структура обернута в `Arc`,
    // так как необходимо будет запускать несколько `task` которые будут владеть этой структурой
    inner: Vec<Arc<PythonSafeObject>>,
}
#[pymethods]
impl Validator {
    #[new]
    /// Конструктор для валидатора, принимает лист классов ошибок
    fn py_constructor(errors_list: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            // Создаем коллекцию классов для будущего построения ошибок на их основе
            let mut class_templates: Vec<Arc<PythonSafeObject>> = Vec::new();
            // Получаем каждый класс как `PyAny` и итеративно обрабатываем его
            for boxed_class in errors_list.downcast::<types::PyList>(py)?.iter() {
                // Переводим из `PyAny` в `PyType`, чтобы получить доступ к атрибутам
                let unboxed_class = boxed_class.downcast::<types::PyType>()?;
                // Сохраняем структуру как `Arc` для `async fn`
                // Сразу получаем все нужный атрибуты от каждого класса по отдельности
                class_templates.push(Arc::new(PythonSafeObject {
                    // Обратно возращаем класс в `PyObject`, чтобы можно было в будущем использовать его снова
                    original_class: unboxed_class.to_object(py),
                    // Получаем все `**extra`
                    extra: init::extra::extra_from_class(unboxed_class, EXTRA_FOR_ERROR_PY)?,
                    // Получаем все регулярные выражения
                    rgxs: init::regex::regex_from_class(unboxed_class)?,
                }));
            }
            Ok(Validator {
                inner: class_templates,
            })
        })
    }
}
