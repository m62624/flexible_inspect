use super::*;

/// Проверяем строку, является ли корректным регулярным выражением (`default regex`)\
/// **Является первым шагом проверки во время инициализаций конструктора**
pub fn is_default_regex_fisrt_step(line: &str) -> bool {
    regex::Regex::new(line).is_ok()
}

/// Проверяем строку, является ли корректным регулярным выражением (`fancy regex`)\
/// **Является вторым шагом проверки во время инициализаций конструктора**
pub fn is_fancy_regex_second_step(line: &str) -> bool {
    fancy_regex::Regex::new(&line).is_ok()
}
