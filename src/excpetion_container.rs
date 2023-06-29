use super::rule::Rule;
use super::*;

#[derive(Debug)]
pub struct ExceptionContainer {
    py_class: PyObject,
    default_roots_set: Option<regex::RegexSet>,
    default_roots_vec: Option<Vec<Rule>>,
    fancy_root_vec: Option<Vec<Rule>>,
}

mod rules_pulling_from_class {
    use super::*;

    impl ExceptionContainer {
        pub fn new(py: Python, py_class: PyObject) -> PyResult<Self> {
            if let Ok(class_py) = py_class.downcast::<types::PyType>(py) {
                if let Ok(py_list) = class_py.getattr(RULES_FROM_CLASS_PY) {
                    if let Ok(py_list) = py_list.downcast::<types::PyList>() {
                        let mut default_roots = Vec::new();
                        let mut fancy_roots = Vec::new();
                        py_list
                            .iter()
                            .map(|rule| {
                                if let Ok(rule) = rule.extract::<rule::Rule>() {
                                    match rule.get_str_raw()? {
                                        rule::RegexRaw::DefaultR(_) => default_roots.push(rule),
                                        rule::RegexRaw::FancyR(_) => fancy_roots.push(rule),
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
                        return Ok(Self {
                            py_class: py_class,
                            default_roots_set: match !&default_roots.is_empty() {
                                true => Some(regex::RegexSet::new(&default_roots).unwrap()),
                                false => None,
                            },
                            fancy_root_vec: match !&fancy_roots.is_empty() {
                                true => Some(fancy_roots),
                                false => None,
                            },
                            default_roots_vec: match !&default_roots.is_empty() {
                                true => Some(default_roots),
                                false => None,
                            },
                        });
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
            } else {
                return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                    "'{}' must be a 'Class'",
                    py_class
                )));
            }
        }
    }
}

mod getters {
    use super::*;

    impl ExceptionContainer {
        pub fn get_py_class(&self) -> &PyObject {
            &self.py_class
        }
        pub fn get_fancy_roots_vec(&self) -> &Option<Vec<Rule>> {
            &self.fancy_root_vec
        }
        pub fn get_default_roots_vec(&self) -> &Option<Vec<Rule>> {
            &self.default_roots_vec
        }
        fn get_roots_set(&self) -> &Option<regex::RegexSet> {
            &self.default_roots_set
        }
    }
}
