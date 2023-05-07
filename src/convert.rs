use super::*;
use pyo3::exceptions;

/// Проверка и конвертация байтов в строку (`UTF-8`)
pub fn bytes_to_string_utf8(bytes: &[u8]) -> PyResult<String> {
    match String::from_utf8(bytes.into()) {
        Ok(result) => Ok(result),
        Err(_) => Err(PyErr::new::<exceptions::PyValueError, _>(format!(
            "{:#?} is not a valid UTF-8 string",
            bytes
        ))),
    }
}

// Функций для конвертация строк в определенный `regex`, почему в них нет проверки на `Err` ?
// В конструкторе TemplateValidator, изначально происходит проверка на корректность строк,
// а значит, можно позже произвести конвертацию сразу, когда необходим будет `regex` для валидаций,

/// Конвертация `String` в `default regex`
pub fn string_to_default_regex(line: String) -> regex::Regex {
    regex::Regex::new(&line).unwrap()
}

/// Конвертация `String` в `fancy regex`
pub fn string_to_fancy_regex(line: String) -> fancy_regex::Regex {
    fancy_regex::Regex::new(&line).unwrap()
}
