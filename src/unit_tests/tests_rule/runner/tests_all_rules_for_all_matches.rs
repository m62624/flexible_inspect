use crate::rule::next::NextStep;

use super::*;

/// Проверяем, что не сработает для всех совпадений, \d{4} != [132] (простые правила)
#[test]
fn test_all_rules_for_all_match_t_0() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [12346] [132] [1234] ] [ [123456789] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![
                Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?.extend_t(
                    py,
                    vec![
                        Rule::spawn(r"\d+", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"\d{4}", MatchRequirement::MustBeFound)?,
                    ],
                )?,
            ],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Error(None));
        Ok(())
    })
}

/// Проверяем, что сработают все правила для всех совпадений (простые правила)
#[test]
fn test_all_rules_for_all_match_t_1() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [12346] [132] [1234] ] [ [123456789] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![
                Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?.extend_t(
                    py,
                    vec![
                        Rule::spawn(r"\d+", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"\d{3}", MatchRequirement::MustBeFound)?,
                    ],
                )?,
            ],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Finish);
        Ok(())
    })
}

/// Проверяем, что не сработают все правила для всех совпадений, \d(?=A) (сложное правило)
#[test]
fn test_all_rules_for_all_match_t_2() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [12346] [132] [1234] ] [ [123456789] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![
                Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?.extend_t(
                    py,
                    vec![
                        Rule::spawn(r"\d+", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"\d(?=A)", MatchRequirement::MustBeFound)?,
                    ],
                )?,
            ],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Error(None));
        Ok(())
    })
}

/// Проверяем, что сработают все правила для всех совпадений (сложное правило)
#[test]
fn test_all_rules_for_all_match_t_3() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [12346] [132] [1234] ] [ [123456789] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![
                Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?.extend_t(
                    py,
                    vec![
                        Rule::spawn(r"\d+", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"\d(?=\d+)", MatchRequirement::MustBeFound)?,
                    ],
                )?,
            ],
        )?;
        assert_eq!(Rule::run(&rule, text), NextStep::Finish);
        Ok(())
    })
}
