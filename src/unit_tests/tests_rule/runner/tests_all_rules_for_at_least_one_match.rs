use super::*;
use crate::rule::next::NextStep;

/// Проверяем, что сработают все правила на одно совпадение (простые правила)
#[test]
fn test_all_rules_for_at_least_one_match_t_0() -> PyResult<()> {
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
                        Rule::spawn(r"\d{4}", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_all_rules_for_at_least_one_match()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Finish);
        Ok(())
    })
}

/// Проверяем, что не сработают все правила на одно совпадение, A != [132] (простые правила)
#[test]
fn test_all_rules_for_at_least_one_match_t_1() -> PyResult<()> {
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
                        Rule::spawn(r"\d{3}", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_all_rules_for_at_least_one_match()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Error(None));
        Ok(())
    })
}

/// Проверяем, что сработают все правила на одно совпадение (сложные правила)
#[test]
fn test_all_rules_for_at_least_one_match_t_2() -> PyResult<()> {
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
                        Rule::spawn(r"\d(?=\d+)", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_all_rules_for_at_least_one_match()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Finish);
        Ok(())
    })
}

/// Проверяем, что не сработают все правила на одно совпадение, \d(?=\d+) != [1] (сложные правила)
#[test]
fn test_all_rules_for_at_least_one_match_t_3() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [2] [1] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"A", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"\d(?=\d+)", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_all_rules_for_at_least_one_match()],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Error(None));
        Ok(())
    })
}
