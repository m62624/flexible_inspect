use super::mock_obj::CustomClassError;
use super::*;

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

    /// Создаем правило с помощью конструктора `Regex` (MatchRequirement::MustNotBefound)
    #[test]
    fn new_t_1() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\w", MatchRequirement::MustNotBefound)?);
        Ok(())
    }

    /// Создаем правило с помощью конструктора `Fancy Regex` (MatchRequirement::MustBeFound)
    #[test]
    fn new_t_2() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\w(?=:D)", MatchRequirement::MustBeFound)?);
        Ok(())
    }

    /// Создаем правило с помощью конструктора `Fancy Regex` (MatchRequirement::MustNotBefound)
    #[test]
    fn new_t_3() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"\w(?=:D)", MatchRequirement::MustNotBefound)?);
        Ok(())
    }

    /// Создаем правило с помощью конструктора, Invalid Regex
    #[test]
    #[should_panic]
    fn new_e_0() {
        pyo3::prepare_freethreaded_python();
        dbg!(Rule::spawn(r"(?P<invalid)", MatchRequirement::MustNotBefound).unwrap());
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

/// Проверка метода отобранных правил
mod fn_get_selected_rules {
    use super::*;

    /// Проверяем отобранные правила, `Regex` & `Fancy Regex` (MatchRequirement::MustBeFound)
    /// Получаем только два правила (`Regex`), и исключаем одно правило (`Fancy Regex`)
    #[test]
    fn get_selected_rules_t_0() -> PyResult<()> {
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
            let subrules = extended_rule.content_unchecked().subrules.as_ref().unwrap();
            assert_eq!(
                Rule::get_selected_rules(
                    &subrules.simple_rules.as_ref().unwrap().regex_set,
                    "[qwe] cxa a:D"
                )
                .len(),
                2
            );
            Ok(())
        })
    }
}

/// Проврека, если в моменте вызова функции extend
/// в теле `Rule` указано `None`, то вызывается функция option_error с причиной
mod fn_option_error {
    use super::*;

    /// Забираем тело `Rule`, ожидаем ошибку (`get_content`)
    #[test]
    #[should_panic]
    fn get_content_e_0() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let mut rule = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound).unwrap();
            std::mem::take(&mut rule);
            rule.extend_t(py, vec![Default::default()]).unwrap();
        });
    }

    /// Забираем тело `Rule`, ожидаем ошибку (`get_content_mut`)
    #[test]
    #[should_panic]
    fn get_content_e_1() {
        pyo3::prepare_freethreaded_python();
        let mut rule = Rule::spawn(r"(?:.+)", MatchRequirement::MustBeFound).unwrap();
        std::mem::take(&mut rule);
        rule.content_unchecked();
    }
}

mod fn_run {
    use super::*;

    #[test]
    fn run_t_0() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let text = "[1234] test test gl gl";
            let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
                py,
                vec![
                    Rule::spawn(r"glg", MatchRequirement::MustNotBefound)?,
                    Rule::spawn(r"\[\d+\]", MatchRequirement::MustBeFound)?,
                    Rule::spawn(r"gl (?=gl)", MatchRequirement::MustBeFound)?,
                ],
            )?;
            let obj = mock_obj::make_obj(py, "found : {data}", None);
            dbg!(Rule::run(&rule, text));
            Ok(())
        })
    }
}
