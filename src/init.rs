use super::*;
use pyo3::exceptions;

/// Расфасовка всех элементов из `PyObject` в структуры
pub fn data_unpackaging(
    py: Python,
    obj: PyObject,
    python_classes: &mut HashMap<usize, ShotStatus>,
    all_simple_rules: &mut HashMap<String, usize>,
    all_hard_rules: &mut HashMap<String, usize>,
    selected_simple_rules: &mut Vec<String>,
) -> PyResult<()> {
    if let Ok(dict) = obj.downcast::<types::PyList>(py) {
        let mut id_class = 0;
        for (key, value) in dict.iter() {
            if let Ok(class_py) = key.downcast::<types::PyType>() {
                get_any_regex_from_class(
                    class_py,
                    id_class,
                    all_simple_rules,
                    all_hard_rules,
                    selected_simple_rules,
                )?;
                if let Ok(flag) = value.extract::<IfFound>() {
                    python_classes.insert(id_class, ShotStatus::new(value.to_object(py), flag));
                } else {
                    return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                        "'{}' must be a 'IfFound'",
                        value
                    )));
                }
                // повышаем id_class для следующего класса
                id_class += 1;
            } else {
                return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                    "'{}' must be a 'Class'",
                    key
                )));
            }
        }
    } else {
        return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "'{}' must be a 'dict =  {{class(Error): enum (IfFound)}}'",
            RULES_FROM_CLASS_PY
        )));
    }
    Ok(())
}

pub fn get_any_regex_from_class(
    class_py: &types::PyType,
    id_class: usize,
    all_simple_rules: &mut HashMap<String, usize>,
    all_hard_rules: &mut HashMap<String, usize>,
    selected_simple_rules: &mut Vec<String>,
) -> PyResult<()> {
    let rgxs: Vec<String> = class_py
        .getattr(RULES_FROM_CLASS_PY)
        .unwrap()
        .extract()
        .map_err(|_| {
            PyErr::new::<exceptions::PyAttributeError, _>(format!(
                "Class must have a '{}' attribute containing a list of regex strings",
                RULES_FROM_CLASS_PY
            ))
        })?;
    for rgx in rgxs {
        if check::is_default_regex_fisrt_step(&rgx) {
            all_simple_rules.insert(rgx.to_string(), id_class);
            selected_simple_rules.push(rgx);
        } else if check::is_fancy_regex_second_step(&rgx) {
            all_hard_rules.insert(rgx, id_class);
        } else {
            return Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                "{} --- Invalid regular expression",
                rgx
            )));
        }
    }
    Ok(())
}
