use super::*;
use crate::{captures::CaptureData, rule::next::NextStep};

/// Проверка next_or_error, где requirement - MustBeFound, Captures - TRUE, Subrules - TRUE,
/// + counter
#[test]
pub fn next_or_error_t_0() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?
            .counter_is_equal(2)
            .extend_t(
                py,
                vec![Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(2)],
            )?;
        let mut captures = CaptureData::find_captures(&rule, "lock");
        let err_data = captures.hashmap_for_error.clone();
        assert_eq!(
            Rule::next_or_data_for_error(&rule, &mut captures),
            NextStep::Error(Some(err_data)),
        );
        Ok(())
    })
}

/// Проверка next_or_error, где requirement - MustBeFound, Captures - TRUE, Subrules - FALSE,
/// + counter
#[test]
pub fn next_or_error_t_1() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(2);
    let mut captures = CaptureData::find_captures(&rule, "lock");
    let err_data = captures.hashmap_for_error.clone();
    assert_eq!(
        Rule::next_or_data_for_error(&rule, &mut captures),
        NextStep::Error(Some(err_data)),
    );
    Ok(())
}

/// Проверка next_or_error, где requirement - MustBeFound, Captures - FALSE, Subrules - TRUE
/// + counter
#[test]
pub fn next_or_error_t_3() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?
            .counter_is_equal(2)
            .extend_t(
                py,
                vec![Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(2)],
            )?;
        let mut captures = CaptureData::find_captures(&rule, "");
        assert_eq!(
            Rule::next_or_data_for_error(&rule, &mut captures),
            NextStep::Error(None),
        );
        Ok(())
    })
}

/// Проверка next_or_error, где requirement - MustBeFound, Captures - FALSE, Subrules - FALSE
/// + counter
#[test]
pub fn next_or_error_t_4() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(2);
    let mut captures = CaptureData::find_captures(&rule, "");
    assert_eq!(
        Rule::next_or_data_for_error(&rule, &mut captures),
        NextStep::Error(None),
    );
    Ok(())
}

/// Проверка next_or_error, где requirement - MustNotBeFound, Captures - TRUE, Subrules - TRUE,
/// + counter
#[test]
pub fn next_or_error_t_5() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustNotBeFound)?
            .counter_is_equal(2)
            .extend_t(
                py,
                vec![Rule::spawn(r"(\w+)", MatchRequirement::MustNotBeFound)?.counter_is_equal(2)],
            )?;
        let mut captures = CaptureData::find_captures(&rule, "lock");
        let err_data = captures.hashmap_for_error.clone();
        assert_eq!(
            Rule::next_or_data_for_error(&rule, &mut captures),
            NextStep::Error(Some(err_data)),
        );
        Ok(())
    })
}

/// Проверка next_or_error, где requirement - MustNotBeFound, Captures - TRUE, Subrules - FALSE,
/// + counter
#[test]
pub fn next_or_error_t_6() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustNotBeFound)?.counter_is_equal(2);
    let mut captures = CaptureData::find_captures(&rule, "lock");
    let err_data = captures.hashmap_for_error.clone();
    assert_eq!(
        Rule::next_or_data_for_error(&rule, &mut captures),
        NextStep::Error(Some(err_data)),
    );
    Ok(())
}

/// Проверка next_or_error, где requirement - MustNotBeFound, Captures - FALSE, Subrules - TRUE
/// + counter
#[test]
pub fn next_or_error_t_7() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustNotBeFound)?
            .counter_is_equal(2)
            .extend_t(
                py,
                vec![Rule::spawn(r"(\w+)", MatchRequirement::MustNotBeFound)?.counter_is_equal(2)],
            )?;
        let mut captures = CaptureData::find_captures(&rule, "");
        let err_data = captures.hashmap_for_error.clone();
        assert_eq!(
            Rule::next_or_data_for_error(&rule, &mut captures),
            NextStep::Error(Some(err_data)),
        );
        Ok(())
    })
}

/// Проверка next_or_error, где requirement - MustNotBeFound, Captures - FALSE, Subrules - FALSE
/// + counter
#[test]
pub fn next_or_error_t_8() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustNotBeFound)?.counter_is_equal(2);
    let mut captures = CaptureData::find_captures(&rule, "");
    let err_data = captures.hashmap_for_error.clone();
    assert_eq!(
        Rule::next_or_data_for_error(&rule, &mut captures),
        NextStep::Error(Some(err_data)),
    );
    Ok(())
}
