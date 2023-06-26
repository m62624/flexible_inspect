use super::*;
use rule::Rule;
#[derive(Debug)]
/// --> TemplateValidator
pub struct ExceptionContainer {
    inner: PyObject,
    default_r: Vec<Rule>,
    fancy_r: Vec<Rule>,
}

impl ExceptionContainer {
    pub fn new(py: Python, py_class: PyObject) -> PyResult<Self> {
        if let Ok(class_py) = py_class.downcast::<types::PyType>(py) {
            let mut default_r = Vec::new();
            let mut fancy_r = Vec::new();
            Self::get_rules(class_py, &mut default_r, &mut fancy_r)?;
            return Ok(Self {
                inner: py_class,
                default_r,
                fancy_r,
            });
        } else {
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "'{}' must be a 'Class'",
                py_class
            )));
        }
    }
    /// Получение всех правил из класса
    fn get_rules(
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
    pub fn get_selected_rules(&self, text: &str) -> Option<Vec<&Rule>> {
        if self.default_r.is_empty() {
            return None;
        }
        return Some(
            regex::RegexSet::new(
                self.default_r
                    .iter()
                    .filter_map(|rule| {
                        if let Some(value) = rule.get_op_str_raw() {
                            return Some(value.get_str());
                        }
                        None
                    })
                    .collect::<Vec<&Box<str>>>(),
            )
            .unwrap()
            .matches(text)
            .iter()
            .map(|id| &self.default_r[id])
            .collect::<Vec<_>>(),
        );
    }
    pub fn get_fancy_rules(&self) -> &Vec<Rule> {
        &self.fancy_r
    }
    pub fn get_obj(&self) -> &PyObject {
        &self.inner
    }
}
