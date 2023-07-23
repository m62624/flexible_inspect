use super::*;

/// Проверка конструктора на корректное регулярное выражение байтов
#[test]
fn new_t_0() {
    RuleBytes::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound);
}

/// Проверка конструктора, ожидаем ошибку из за некорректного регулярного выражения байтов
#[test]
#[should_panic(expected = "regular expression is incorrect")]
fn new_t_1() {
    RuleBytes::new(r"(?-u)(?<cstr>[^\x00]+\x00", MatchRequirement::MustBeFound);
}