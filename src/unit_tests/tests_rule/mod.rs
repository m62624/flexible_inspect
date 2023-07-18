use super::*;
mod runner;
mod tests_auto_generate_rule;
mod tests_eq;
mod tests_next;
use crate::unit_tests::mock_obj::CustomClassError;

/// Проверка конструктора `Rule`
mod fn_new {
    use super::*;

    /// Создаем правило с помощью конструктора `Regex` (MatchRequirement::MustBeFound)
    #[test]
    fn new_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\d", MatchRequirement::MustBeFound)?);
        Ok(())
    }

    /// Создаем правило с помощью конструктора `Regex` (MatchRequirement::MustNotBeFound)
    #[test]
    fn new_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\w", MatchRequirement::MustNotBeFound)?);
        Ok(())
    }

    /// Создаем правило с помощью конструктора `Fancy Regex` (MatchRequirement::MustBeFound)
    #[test]
    fn new_t_2() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)?);
        Ok(())
    }

    /// Создаем правило с помощью конструктора `Fancy Regex` (MatchRequirement::MustNotBeFound)
    #[test]
    fn new_t_3() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\w(?=:D)", MatchRequirement::MustNotBeFound)?);
        Ok(())
    }

    /// Создаем правило с помощью конструктора, Invalid Regex
    #[test]
    #[should_panic]
    fn new_e_0() {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"(?P<invalid)", MatchRequirement::MustNotBeFound).unwrap());
    }
}

/// Проверка расширения `Rule`
mod fn_extend {
    use super::*;

    /// Расширяем правило `Regex` & `Fancy Regex` (MatchRequirement::MustBeFound)
    #[test]
    fn extend_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let mut rule = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound)?;
            let extended_rule = rule.extend_t(
                py,
                vec![
                    Rule::spawn(r"\[.+\]", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"\w", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)?,
                ],
            )?;
            dbg!(extended_rule);
            Ok(())
        })
    }

    /// Расширяем правило `Fancy Regex` (MatchRequirement::MustBeFound)
    #[test]
    fn extend_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| -> PyResult<()> {
            let mut rule = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound)?;
            let extended_rule = rule.extend_t(
                py,
                vec![Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)?],
            )?;
            dbg!(extended_rule);
            Ok(())
        })
    }

    /// Расширяем правило, ожидаем ошибку, где указывается от какого корня
    /// произошла ошибка
    #[test]
    #[should_panic]
    fn extend_e_0() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let mut rule = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound).unwrap();
            rule.extend(
                py,
                types::PyList::new(
                    py,
                    vec![
                        Rule::spawn(r"\[.+\]", MatchRequirement::MustBeFound)
                            .unwrap()
                            .into_py(py),
                        Rule::spawn(r"\w", MatchRequirement::MustBeFound)
                            .unwrap()
                            .into_py(py),
                        Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)
                            .unwrap()
                            .into_py(py),
                        types::PyType::new::<mock_obj::CustomClassError>(py).into(),
                    ],
                )
                .into_py(py),
            )
            .unwrap();
        });
    }

    /// Расширяем правило, ожидаем ошибку, где указывается от какого корня
    #[test]
    #[should_panic]
    fn extend_e_1() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let mut rule = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound).unwrap();
            rule.extend(py, types::PyType::new::<CustomClassError>(py).into_py(py))
                .unwrap();
        });
    }
}
