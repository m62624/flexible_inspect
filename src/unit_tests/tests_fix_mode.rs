use crate::captures::CaptureData;

use super::*;

#[test]
fn test_all_rules_all_match_fix_1() -> PyResult<()> {
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
        dbg!(Rule::run(&rule, text));
        Ok(())
    })
}

#[test]
fn test_at_least_one_rule_for_all_matches_fix() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"\d{10}", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"\d(?=\d+)", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"\d{4}", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_all_matches()],
        )?;
        dbg!(Rule::run(&rule, text));
        Ok(())
    })
}

#[test]
fn test_all_rules_for_at_least_one_match_fix() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"\d{10}", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"\d(?=\d+)", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"\d{4}", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_all_rules_for_at_least_one_match()],
        )?;
        dbg!(Rule::run(&rule, text));
        Ok(())
    })
}

#[test]
fn test_at_least_one_rule_for_at_least_one_match_fix() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
        let rule = Rule::spawn(r".+", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![Rule::spawn(r"\[[^\[\]]+\]", MatchRequirement::MustBeFound)?
                .extend_t(
                    py,
                    vec![
                        Rule::spawn(r"\d{10}", MatchRequirement::MustNotBeFound)?,
                        Rule::spawn(r"\d(?=\d+)", MatchRequirement::MustBeFound)?,
                        Rule::spawn(r"\d{4}", MatchRequirement::MustBeFound)?,
                    ],
                )?
                .mode_at_least_one_rule_for_at_least_one_match()],
        )?;
        dbg!(Rule::run(&rule, text));
        Ok(())
    })
}
