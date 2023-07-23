use super::*;

/// Проверка конструктора на корректное регулярное выражение Default Regex
#[test]
fn new_t_0() {
    Rule::new(r".*", MatchRequirement::MustBeFound);
}

/// Проверка конструктора, ожидаем ошибку из за некорректного регулярного выражения Default Regex
#[test]
#[should_panic(expected = "regular expression is incorrect")]
fn new_t_1() {
    Rule::new(r"\j", MatchRequirement::MustBeFound);
}

/// Проверка конструктора на корректное регулярное выражение Fancy Regex
#[test]
fn new_t_2() {
    Rule::new(r"\d+(?=$)", MatchRequirement::MustBeFound);
}

/// Проверка конструктора, ожидаем ошибку из за некорректного регулярного выражения Fancy Regex
#[test]
#[should_panic(expected = "regular expression is incorrect")]
fn new_t_3() {
    Rule::new(r"\d+(?=$", MatchRequirement::MustBeFound);
}
