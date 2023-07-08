use super::rule::slice::RuleContext;
use super::rule::{SimpleRules, Subrules as RootRules};
use super::*;
mod getter;
mod runner;

/// `Cartridge` - структура для хранения правил и тело класса для создания ошибки
#[derive(Debug)]
pub struct Cartridge {
    pub py_class: PyObject,
    root_rules: RootRules,
}
/// `CartridgeWrapper` - структура для хранения `Cartridge` в `Arc`, для безопасной передачи между `async tasks`
#[derive(Debug)]
pub struct CartridgeWrapper(Arc<Cartridge>);

impl CartridgeWrapper {
    /// Конструктор для `CartridgeWrapper`
    pub fn new(py: Python, py_class: PyObject) -> PyResult<Self> {
        // Проверяем, что это класс
        if let Ok(class_py) = py_class.downcast::<types::PyType>(py) {
            // Проверяем, что у класса есть атрибут `RULES_FROM_CLASS_PY`
            if let Ok(py_list) = class_py.getattr(RULES_FROM_CLASS_PY) {
                // Проверяем, что это список
                if let Ok(py_list) = py_list.downcast::<types::PyList>() {
                    // Создаем вектор для простых правил
                    let mut simple_rules = Vec::new();
                    // Создаем вектор для сложных правил
                    let mut complex_rules = Vec::new();
                    // Делим правила на простые и сложные
                    RuleContext::slice_rules(
                        RuleContext::Root(class_py),
                        py_list,
                        &mut simple_rules,
                        &mut complex_rules,
                    )?;
                    // Возвращаем структуру
                    Ok(Self(Arc::new(Cartridge {
                        py_class,
                        root_rules: RootRules::new(SimpleRules::new(simple_rules), complex_rules),
                    })))
                } else {
                    let err_msg = format!("'{}' must be a 'List [ Rule, Rule... ] '", py_list);
                    // ================= (LOG) =================
                    error!("{}", err_msg);
                    // =========================================
                    // Если это не список, то возвращаем ошибку
                    Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg))
                }
            } else {
                let err_msg = format!(
                    "The class `{}` has no attribute : `{}`",
                    class_py, RULES_FROM_CLASS_PY
                );
                // ================= (LOG) =================
                error!("{}", err_msg);
                // =========================================
                // Если у класса нету атрибута `RULES_FROM_CLASS_PY`, то возвращаем ошибку
                Err(PyErr::new::<exceptions::PyAttributeError, _>(err_msg))
            }
        } else {
            let err_msg = format!("'{}' must be a 'Class'", py_class);
            // ================= (LOG) =================
            error!("{}", err_msg);
            // =========================================
            // Если это не класс, то возвращаем ошибку
            Err(PyErr::new::<exceptions::PyTypeError, _>(err_msg))
        }
    }
}
