use super::*;

/// Модуль для проверки
pub mod check {
    use super::*;
    /// Проверяем строку, является ли корректным регулярным выражением (`default regex`)\
    /// ** Является первым шагом проверки во время инициализаций конструктора **
    pub fn is_default_regex_fisrt_step(line: &str) -> bool {
        regex::Regex::new(line).is_ok()
    }

    /// Проверяем строку, является ли корректным регулярным выражением (`fancy regex`)\
    /// ** Является вторым шагом проверки во время инициализаций конструктора **
    pub fn is_fancy_regex_second_step(line: &str) -> bool {
        fancy_regex::Regex::new(line).is_ok()
    }

    /// Проверка и конвертация в байты
    pub fn is_valid_byte_utf8(bytes: &[u8]) -> PyResult<String> {
        Ok(String::from_utf8(bytes.into())?)
    }
}
/// Модуль для конвертации
pub mod convert {

    // Функций для конвертация строк в определенный `regex`, почему в них нет проверки на `Err` ?

    // В конструкторе `Rule`, изначально происходит проверка на корректность строк,
    // а значит, можно позже произвести конвертацию сразу, когда необходим будет `regex` для валидаций,

    /// Конвертация `&str` в `default regex`
    pub fn str_to_default_regex(line: &str) -> regex::Regex {
        regex::Regex::new(line).unwrap()
    }

    /// Конвертация `&str в `fancy regex`
    pub fn str_to_fancy_regex(line: &str) -> fancy_regex::Regex {
        fancy_regex::Regex::new(line).unwrap()
    }

    /// Конвертация `regex::Captures` в `Vec<&str>`
    pub fn str_from_default_capture<'a>(captures: &'a Vec<regex::Captures<'a>>) -> Vec<&'a str> {
        captures
            .iter()
            .flat_map(|capture| {
                capture
                    .iter()
                    .filter_map(|capture| capture.map(|value| value.as_str()))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Конвертация `fancy_regex::Captures` в `Vec<&str>`
    pub fn str_from_fancy_capture<'a>(
        captures: &'a Vec<fancy_regex::Captures<'a>>,
    ) -> Vec<&'a str> {
        captures
            .iter()
            .flat_map(|capture| {
                capture
                    .iter()
                    .filter_map(|capture| capture.map(|value| value.as_str()))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
