use super::*;
use pyo3::exceptions;
// Расфасовка данных
pub fn data_unpackaging(
    py: Python,
    obj: PyObject,
    python_classes: &mut HashMap<usize, PyObject>,
    all_simple_rules: &mut HashMap<String, RuleStatus>,
    all_hard_rules: &mut HashMap<String, RuleStatus>,
    selected_simple_rules: &mut Vec<String>,
) -> PyResult<()> {
    // Проверяем что переданый объект является списком, иначе ошибка типа переменной
    if let Ok(dict) = obj.downcast::<types::PyList>(py) {
        let mut id_class: usize = 0;
        for class_py in dict {
            // Проверяем что элемент списка является классом, иначе ошибка типа переменной
            if let Ok(class_py) = class_py.downcast::<types::PyType>() {
                get_any_regex_from_class(
                    class_py,
                    id_class,
                    all_simple_rules,
                    all_hard_rules,
                    selected_simple_rules,
                )?;
                python_classes.insert(id_class, class_py.to_object(py));

                id_class += 1;
            } else {
                return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                    "'{}' must be a 'Class'",
                    class_py
                )));
            }
        }
    } else {
        return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "'{}' must be a 'List[ Class, Class... ]'",
            obj
        )));
    }
    Ok(())
}

pub fn get_any_regex_from_class(
    class_py: &types::PyType,
    id_class: usize,
    all_simple_rules: &mut HashMap<String, RuleStatus>,
    all_hard_rules: &mut HashMap<String, RuleStatus>,
    selected_simple_rules: &mut Vec<String>,
) -> PyResult<()> {
    // Проверяем что класс имеет атрибут RULES_FROM_CLASS_PY, иначе ошибка атрибута
    if let Ok(py_dict) = class_py.getattr(RULES_FROM_CLASS_PY) {
        // Проверяем что атрибут является словарем, иначе ошибка типа переменной
        if let Ok(dict) = py_dict.downcast::<types::PyDict>() {
            for (key, value) in dict {
                // Проверяем что ключ является строкой, иначе ошибка типа переменной
                if let Ok(key) = key.extract::<String>() {
                    // Проверяем что значение является Enum, иначе ошибка типа переменной
                    if let Ok(value) = value.extract::<It>() {
                        // Проверяем что ключ является валидным регулярным выражением (простого типа)
                        if check_convert::check::is_default_regex_fisrt_step(&key) {
                            all_simple_rules
                                .insert(key.to_string(), RuleStatus::new(id_class, value));
                            selected_simple_rules.push(key);
                            // Проверяем что ключ является валидным регулярным выражением (сложного типа)
                        } else if check_convert::check::is_fancy_regex_second_step(&key) {
                            all_hard_rules
                                .insert(key.to_string(), RuleStatus::new(id_class, value));
                        } else {
                            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                                "{} --- Invalid regular expression",
                                key
                            )));
                        }
                    } else {
                        return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                            "'{}' must be a 'Enum'",
                            value
                        )));
                    }
                } else {
                    return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "'{}' must be a 'String'",
                        key
                    )));
                }
            }
        } else {
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'dict {{ Class: Enum }}'",
                py_dict
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
