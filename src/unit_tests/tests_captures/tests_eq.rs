use super::captures::*;
use super::*;

/// Проверка равенства CaptureType::Single с одинаковыми значениями
#[test]
fn test_captures_eq_t_0() -> PyResult<()> {
    let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
    let text = "lock 1";
    let capture_0 = CaptureData::find_captures(&rule, text);
    let capture_1 = CaptureData::find_captures(&rule, text);
    assert_eq!(capture_0, capture_1);
    Ok(())
}

/// Проверка равенства CaptureType::Single с разными значениями
#[test]
fn test_captures_eq_t_1() -> PyResult<()> {
    let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
    let text = "lock 1";
    let capture_0 = CaptureData::find_captures(&rule, text);
    let capture_1 = CaptureData::find_captures(&rule, "lock 2");
    assert_ne!(capture_0, capture_1);
    Ok(())
}

/// Проверка равенства CaptureType::Multiple с одинаковыми значениями

#[test]
fn test_captures_eq_t_2() -> PyResult<()> {
    let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.duplicate_matches();
    let text = "lock lock";
    let capture_0 = CaptureData::find_captures(&rule, text);
    let capture_1 = CaptureData::find_captures(&rule, text);
    assert_eq!(capture_0, capture_1);
    Ok(())
}

/// Проверка равенства CaptureType::Multiple с разными значениями
#[test]
fn test_captures_eq_t_3() -> PyResult<()> {
    let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.duplicate_matches();
    let text = "lock lock";
    let capture_0 = CaptureData::find_captures(&rule, text);
    let capture_1 = CaptureData::find_captures(&rule, "lock lock lock");
    assert_ne!(capture_0, capture_1);
    Ok(())
}
