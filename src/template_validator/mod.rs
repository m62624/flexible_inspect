use super::cartridge::CartridgeWrapper;
use super::custom_error;
use super::init_logger;
use super::rule::next::NextStep;
use super::*;
// =================================

mod traverse_and_clear;
/// Модуль для валидации текста
mod validate;
/// Структура хранит картриджи с правилами и телом класса
#[derive(Debug)]
pub struct TemplateSafeSelf {
    cartridges: Vec<CartridgeWrapper>,
}

/// Структура хранит в `Arc` тело валидатора\
/// Необходимо, так как используется между `async tasks`
#[pyclass]
#[derive(Debug)]
pub struct TemplateValidator(Option<Arc<TemplateSafeSelf>>);

#[pymethods]
impl TemplateValidator {
    /// The Constructor, takes classes (API for `Python`)
    #[new]
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        init_logger();
        let slf = Self(Some(Arc::new(TemplateSafeSelf::new(py, cartridges)?)));
        {
            debug!(
                "loaded classes in the validator: {:#?}",
                if let Some(classes) = &slf.0 {
                    classes
                        .cartridges
                        .iter()
                        .map(|x| x.get_cartridge().get_py_class().to_string())
                        .collect::<Vec<_>>()
                } else {
                    vec![String::from("None")]
                }
            );
        }
        Ok(slf)
    }
}

impl TemplateSafeSelf {
    /// Конструктор, принимает классы, происходит проверка всех получаемых данных
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        // Вектор для хранения классов
        let mut data = Vec::new();
        // Проверка на `List`
        if let Ok(list) = cartridges.downcast::<types::PyList>(py) {
            // Проверка на `Class`
            list.iter().try_for_each(|class_obj| {
                if let Ok(class_py) = class_obj.downcast::<types::PyType>() {
                    data.push(CartridgeWrapper::new(py, class_py.to_object(py))?);
                    Ok(())
                } else {
                    let err_msg = format!("'{}' must be a 'Class'", class_obj);

                    // ================= (LOG) =================
                    error!("(validator constructor): {}", err_msg);
                    // =========================================

                    Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg))
                }
            })?;

            // Возвращаем структуру
            Ok(Self { cartridges: data })
        } else {
            let err_msg = format!("'{}' must be a 'List[ Class, Class... ]'", cartridges);

            // ================= (LOG) =================
            error!("(validator constructor): {}", err_msg);
            // =========================================

            // Если это не `List`, то возвращаем ошибку
            Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg))
        }
    }
}
