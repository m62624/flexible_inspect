use super::rule::{MatchRequirement, Rule};
use super::*;

#[cfg(test)]
mod tests_capture;
#[cfg(test)]
mod tests_cartridge;
#[cfg(test)]
mod tests_modes;
#[cfg(test)]
mod tests_rule;
#[cfg(test)]
mod tests_template_validator;

#[cfg(test)]
pub mod mock_obj {
    use super::*;

    #[pyclass]
    pub struct CustomClassError {}

    #[pymethods]
    impl CustomClassError {
        #[new]
        fn new() -> Self {
            Self {}
        }
    }
    /// Создаем объект `Rule` для тестов
    pub fn make_obj(py: Python, message: &str, rules: Option<Vec<Rule>>) -> PyObject {
        let obj = types::PyType::new::<CustomClassError>(py);
        obj.setattr(
            MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
            types::PyString::new(py, format!("{}", message).as_str()),
        )
        .unwrap();
        obj.setattr(
            RULES_FROM_CLASS_PY,
            match rules {
                Some(rules) => types::PyList::new(py, rules.into_iter().map(|r| r.into_py(py))),
                None => types::PyList::empty(py),
            },
        )
        .unwrap();
        obj.into()
    }
}

mod for_rule {
    use super::*;

    // Реализация методов для тестов
    impl Rule {
        /// ! ONLY FOR TESTS !\
        /// Создаем новый объект `Rule`
        pub fn spawn(pattern: &str, mrq: MatchRequirement) -> PyResult<Rule> {
            Rule::new(String::from(pattern), mrq)
        }

        /// ! ONLY FOR TESTS !\
        /// Расширяем объект `Rule` с помощью переданного списка
        pub fn extend_t(&mut self, py: Python, sub_rules_list: Vec<Rule>) -> PyResult<Self> {
            self.extend(
                py,
                types::PyList::new(py, sub_rules_list.into_iter().map(|r| r.into_py(py)))
                    .into_py(py),
            )
        }
    }
}
