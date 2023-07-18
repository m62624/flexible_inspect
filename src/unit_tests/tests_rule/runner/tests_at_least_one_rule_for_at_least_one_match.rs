use super::*;
use crate::rule::next::NextStep;

/// Проверяем, чтобы хотя бы одно правило сработало для хотя бы одного совпадения (простые правила)
#[test]
fn test_at_least_one_rule_for_at_least_one_match_t_0() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [12345678911] [12345678911] ] [ [12345678911] [1234567811] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"\d{10}", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"x", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"y", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_at_least_one_match()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Finish);
        Ok(())
    })
}

/// Проверяем, чтобы не сработало ни одно правило для какого либо совпадения (простые правила)
#[test]
fn test_at_least_one_rule_for_at_least_one_match_t_1() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [12345678911] [12345678911] [12345678911] ] [ [12345678911] [12345678911] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"12345678911", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"x", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"y", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_at_least_one_match()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Error(None));
        Ok(())
    })
}

/// Проверяем, чтобы хотя бы одно правило сработало для хотя бы одного совпадения (сложные правила)
#[test]
fn test_at_least_one_rule_for_at_least_one_match_t_3() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [12345678911] [A] [12345678911] ] [ [12345678911] [1234567811] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"A(?=\])", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"y", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_at_least_one_match()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Finish);
        Ok(())
    })
}

/// Проверяем, чтобы не сработало ни одно правило для какого либо совпадения (сложные правила)
#[test]
fn test_at_least_one_rule_for_at_least_one_match_t_4() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [12345678911] [A] [12345678911] ] [ [12345678911] [1234567811] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"A(?=\]\])", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"y", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_at_least_one_match()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Error(None));
        Ok(())
    })
}


/// Проверяем, чтобы не сработало ни одно правило для какого либо совпадения (простые правила)
#[test]
fn test_at_least_one_rule_for_at_least_one_match_t_5() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [A] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"A", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"y", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_at_least_one_match()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Error(None));
        Ok(())
    })
}