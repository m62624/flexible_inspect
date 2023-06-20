use super::*;
mod validate;
use pyo3::{exceptions, types};
#[pyclass]
#[derive(Debug, Clone)]
pub struct TemplateValidator {
    #[pyo3(get, set)]
    py_classes: HashMap<usize, PyObject>,
    all_rules: HashMap<rule::Rule, usize>,
}

#[pymethods]
impl TemplateValidator {
    #[new]
    fn new(py: Python, error_classes: PyObject) -> PyResult<Self> {
        // Проверяем, что error_classes - это список
        if let Ok(list) = error_classes.downcast::<types::PyList>(py) {
            let mut py_classes: HashMap<usize, PyObject> = HashMap::new();
            let mut all_rules: HashMap<rule::Rule, usize> = HashMap::new();
            let mut index = 0;
            // Проходимся по всем элементам списка
            for class_py in list {
                // Проверяем, что все элементы списка - это классы
                if let Ok(class_py) = class_py.downcast::<types::PyType>() {
                    // 1 - Сохраняем от каждого класса все rules
                    Self::get_rules(class_py, index, &mut all_rules)?;
                    // 2 - Теперь можем удалить от объектов их rules
                    class_py.delattr(RULES_FROM_CLASS_PY)?;
                    // 3 - Сохраняем тело класса для создания ошибки
                    py_classes.insert(index, class_py.to_object(py));

                    index += 1;
                } else {
                    return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "'{}' must be a 'Class'",
                        class_py
                    )));
                }
            }
            let x = Self {
                py_classes,
                all_rules,
            };
            Ok(x)
        } else {
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'List[ Class, Class... ]'",
                error_classes
            )));
        }
    }
}

impl TemplateValidator {
    fn get_rules(
        class_py: &types::PyType,
        index: usize,
        all_rules: &mut HashMap<rule::Rule, usize>,
    ) -> PyResult<()> {
        if let Ok(py_list) = class_py.getattr(RULES_FROM_CLASS_PY) {
            if let Ok(py_list) = py_list.downcast::<types::PyList>() {
                for rule in py_list {
                    if let Ok(rule) = rule.extract::<rule::Rule>() {
                        all_rules.insert(rule, index);
                    } else {
                        return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                            "'{}' must be a 'Rule' from class `{}`",
                            rule.get_type().name().unwrap(),
                            class_py.name().unwrap()
                        )));
                    }
                }
            } else {
                return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                    "'{}' must be a 'List[ Rule, Rule... ]'",
                    py_list
                )));
            }
        } else {
            return Err(PyErr::new::<exceptions::PyAttributeError, _>(format!(
                "There is no '{}' attribute for getting rules",
                RULES_FROM_CLASS_PY
            )));
        }
        Ok(())
    }
}
