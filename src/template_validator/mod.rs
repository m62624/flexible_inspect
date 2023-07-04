use super::cartridge::CartridgeWrapper;
use super::custom_error;
use super::rule::next::NextStep;
use super::*;
// =================================

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
pub struct TemplateValidator(Arc<TemplateSafeSelf>);

#[pymethods]
impl TemplateValidator {
    /// The Constructor, takes classes (API for `Python`)
    #[new]
    pub fn new(py: Python, cartridges: PyObject) -> PyResult<Self> {
        Ok(Self(Arc::new(TemplateSafeSelf::new(py, cartridges)?)))
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
                    Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "'{}' must be a 'Class'",
                        class_obj
                    )))
                }
            })?;
            // Возвращаем структуру
            Ok(Self { cartridges: data })
        } else {
            // Если это не `List`, то возвращаем ошибку
            Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'List[ Class, Class... ]'",
                cartridges
            )))
        }
    }
}
