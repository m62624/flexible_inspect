use super::*;
use pyo3::{exceptions, types};
impl ExceptionContainer {
    /// Получение всех правил из класса
    pub fn get_all_rules_from_class(
        class_py: &types::PyType,
        default_rules: &mut Vec<Rule>,
        fancy_rules: &mut Vec<Rule>,
    ) -> PyResult<()> {
        // Проверяем наличие атрибута с правилами
        if let Ok(py_list) = class_py.getattr(RULES_FROM_CLASS_PY) {
            // Проверяем, что это список
            if let Ok(py_list) = py_list.downcast::<types::PyList>() {
                py_list
                    .iter()
                    .map(|rule| {
                        if let Ok(rule) = rule.extract::<rule::Rule>() {
                            match rule.get_str_raw()? {
                                rule::RegexRaw::DefaultR(_) => default_rules.push(rule),
                                rule::RegexRaw::FancyR(_) => fancy_rules.push(rule),
                            }
                            Ok(())
                        } else {
                            Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                                "'{}' must be a 'Rule' from class `{}`",
                                rule.get_type().name().unwrap(),
                                class_py.name().unwrap()
                            )))
                        }
                    })
                    .collect::<PyResult<Vec<_>>>()?;
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
