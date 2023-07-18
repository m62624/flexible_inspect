use crate::rule::next::NextStep;

use super::*;

/// Проверяем, что сработает одно правило для всех совпадений (простые правила)
#[test]
fn test_at_least_one_rule_for_all_matches_t_0() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"A", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"B", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"C", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_all_matches()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Finish);
        Ok(())
    })
}

/// Проверяем, что не сработает ни одно правило для всех совпадений (простые правила)

#[test]
fn test_at_least_one_rule_for_all_matches_t_1() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"A", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"B", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"C", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_all_matches()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Error(None));
        Ok(())
    })
}

/// Проверяем, что сработает одно правило для всех совпадений (сложные правила)
#[test]
fn test_at_least_one_rule_for_all_matches_t_3() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"B", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"\d(?=\d+)", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_all_matches()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Finish);
        Ok(())
    })
}

/// Проверяем, что не сработает ни одно правило для всех совпадений (сложные правила)

#[test]
fn test_at_least_one_rule_for_all_matches_t_4() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"B", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"\d(?=C)", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_all_matches()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Error(None));
        Ok(())
    })
}

/// Проверяем, что сработает одно правило для всех совпадений (простые правила)
#[test]
fn test_at_least_one_rule_for_all_matches_t_5() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"\d+", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"B", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"C", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_all_matches()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Finish);
        Ok(())
    })
}
