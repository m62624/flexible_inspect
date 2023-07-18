use crate::captures::CaptureData;

use super::*;

/// Проверка создание ошибки для python без получения переменных с extra
#[test]
fn make_error_t_0() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
        let mock_class = mock_obj::make_obj(py, "lock", Some(vec![rule]));
        assert!(custom_error::make_error(py, &mock_class, &mut None).is_err());
        Ok(())
    })
}

/// Првоерка создания ошибки для python с получением переменных с extra, но без заполнения сообщения
#[test]
fn make_error_t_1() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
        let mut extra_with_value = HashMap::new();
        extra_with_value.insert(String::from("name"), String::from("lock"));
        let mock_class = mock_obj::make_obj(py, "lock", Some(vec![rule]));
        assert!(custom_error::make_error(py, &mock_class, &mut Some(extra_with_value)).is_err());
        Ok(())
    })
}

/// Проверка создания ошибки для python с получением переменных с extra и заполнением сообщения с extra_message
#[test]
fn make_error_t_2() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?;
        let mut extra_with_value = HashMap::new();
        extra_with_value.insert(String::from("group_x"), String::from("lock"));
        let mock_class = mock_obj::make_obj(py, "{group_x}", Some(vec![rule]));
        assert!(custom_error::make_error(py, &mock_class, &mut Some(extra_with_value)).is_err());
        Ok(())
    })
}
