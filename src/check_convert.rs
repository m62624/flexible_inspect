use super::*;
pub mod check {
    /// Проверяем строку, является ли корректным регулярным выражением (`default regex`)\
    /// **Является первым шагом проверки во время инициализаций конструктора**
    pub fn is_default_regex_fisrt_step(line: &str) -> bool {
        regex::Regex::new(line).is_ok()
    }

    /// Проверяем строку, является ли корректным регулярным выражением (`fancy regex`)\
    /// **Является вторым шагом проверки во время инициализаций конструктора**
    pub fn is_fancy_regex_second_step(line: &str) -> bool {
        fancy_regex::Regex::new(line).is_ok()
    }
}
// Модуль для конвертации
pub mod convert {
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
    pub fn string_to_default_regex(line: &str) -> regex::Regex {
        regex::Regex::new(line).unwrap()
    }

    /// Конвертация `String` в `fancy regex`
    pub fn string_to_fancy_regex(line: &str) -> fancy_regex::Regex {
        fancy_regex::Regex::new(line).unwrap()
    }
}
