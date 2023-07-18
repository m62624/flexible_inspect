use super::*;

/// Проверка равенства корневого правила с одинаковыми параметрами
#[test]
pub fn test_rule_eq_t_0() -> PyResult<()> {
    let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
    let rule_1 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
    assert_eq!(rule_0, rule_1);
    Ok(())
}

/// Проверка равенства корневого правила с разными параметрами MatchRequirement
#[test]
pub fn test_rule_eq_t_1() -> PyResult<()> {
    let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
    let rule_1 = Rule::spawn(r"(\w+)", MatchRequirement::MustNotBeFound)?;
    assert_ne!(rule_0, rule_1);
    Ok(())
}

/// Проверка равенства корневого правила с разными параметрами Regex
#[test]
pub fn test_rule_eq_t_2() -> PyResult<()> {
    let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
    let rule_1 = Rule::spawn(r"(\w+ (?=lock))", MatchRequirement::MustBeFound)?;
    assert_ne!(rule_0, rule_1);
    Ok(())
}

/// Проверка равенства корневого правила с одинаковыми счетчиками counter_is_equal
#[test]
pub fn test_rule_eq_t_3() -> PyResult<()> {
    let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(5);
    let rule_1 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(5);
    assert_eq!(rule_0, rule_1);
    Ok(())
}

/// Проверка равенства корневого правила с разными счетчиками counter_is_equal
#[test]
pub fn test_rule_eq_t_4() -> PyResult<()> {
    let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(5);
    let rule_1 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(10);
    assert_ne!(rule_0, rule_1);
    Ok(())
}

/// Проверка равенства корневого правила с одинаковыми счетчиками counter_more_than
#[test]
pub fn test_rule_eq_t_5() -> PyResult<()> {
    let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_more_than(5);
    let rule_1 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_more_than(5);
    assert_eq!(rule_0, rule_1);
    Ok(())
}

/// Проверка равенства корневого правила с разными счетчиками counter_more_than
#[test]
pub fn test_rule_eq_t_6() -> PyResult<()> {
    let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_more_than(5);
    let rule_1 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_more_than(10);
    assert_ne!(rule_0, rule_1);
    Ok(())
}

/// Проверка равенства корневого правила с одинаковыми счетчиками counter_less_than
#[test]
pub fn test_rule_eq_t_7() -> PyResult<()> {
    let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_less_than(5);
    let rule_1 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_less_than(5);
    assert_eq!(rule_0, rule_1);
    Ok(())
}

/// Проверка равенства корневого правила с разными счетчиками counter_less_than
#[test]
pub fn test_rule_eq_t_8() -> PyResult<()> {
    let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_less_than(5);
    let rule_1 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_less_than(10);
    assert_ne!(rule_0, rule_1);
    Ok(())
}

/// Проверка равенства корневого правила с подправилами
#[test]
pub fn test_rule_eq_t_9() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![
                Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?,
                Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?,
            ],
        )?;
        let rule_1 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![
                Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?,
                Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?,
            ],
        )?;
        assert_eq!(rule_0, rule_1);
        Ok(())
    })
}

/// Проверка равенства корневого правила с разными подправилами
#[test]
pub fn test_rule_eq_t_10() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let rule_0 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![
                Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?,
                Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?,
            ],
        )?;
        let rule_1 = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.extend_t(
            py,
            vec![
                Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?,
                Rule::spawn(r"(\w+ 12)", MatchRequirement::MustNotBeFound)?,
            ],
        )?;
        assert_ne!(rule_0, rule_1);
        Ok(())
    })
}
