use super::*;
use pyo3::types;

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
            PyList::new(py, sub_rules_list.into_iter().map(|r| r.into_py(py))).into(),
        )
    }
}

/// Тестируем конструктор
#[cfg(test)]
mod fn_new {
    use super::*;

    /// Тестируем конструктор на правильность работы
    #[test]
    fn constructor_test_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        let info_rule = Rule::new(String::from(r"\d"), MatchRequirement::MustBeFound)?;
        dbg!(&info_rule);
        Ok(())
    }

    /// Тестируем конструктор на ошибку `Invalid Regex`
    #[test]
    #[should_panic]
    fn constructor_test_e_0() {
        pyo3::prepare_freethreaded_python();
        Rule::new(String::from(r"(?P<invalid)"), MatchRequirement::MustBeFound).unwrap();
    }
}

/// Тестируем метод `extend`
#[cfg(test)]
mod fn_extend {
    use super::*;

    /// Тестируем метод `extend` на правильность работы
    #[test]
    fn extend_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let root1 = Rule::spawn(r"[.+]", MatchRequirement::MustBeFound)?.extend_t(
                py,
                vec![
                    Rule::spawn(r"[.+]", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"\d", MatchRequirement::MustNotBefound)?.extend_t(
                        py,
                        vec![Rule::spawn(
                            r"(?=hello)world",
                            MatchRequirement::MustBeFound,
                        )?],
                    )?,
                ],
            );
            dbg!(&root1);
            Ok(())
        })
    }

    /// Тестируем метод `extend` на ошибку `TypeError`, ожидается `PyList`
    #[test]
    #[should_panic]
    fn extend_e_0() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            Rule::spawn(r"[.+]", MatchRequirement::MustBeFound)
                .unwrap()
                .extend(py, types::PyType::new::<Rule>(py).into_py(py))
                .unwrap();
        });
    }

    /// Тестируем метод `extend` на ошибку `TypeError`, ожидается `Rule` в `PyList`
    #[test]
    #[should_panic]
    fn extend_e_1() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            Rule::spawn(r"[.+]", MatchRequirement::MustBeFound)
                .unwrap()
                .extend(py, PyList::new(py, vec![1, 2, 3]).into_py(py))
                .unwrap();
        });
    }
}
