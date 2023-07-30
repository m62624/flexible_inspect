mod tests_captures;
mod tests_extend;
use crate::core::rules::traits::CalculateValueRules;
use crate::core::rules::traits::RuleBase;
use crate::prelude::*;
use std::collections::HashSet;

/// Проверка конструктора на корректное регулярное выражение байтов
#[test]
fn fn_new_t_0() {
    RuleBytes::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound);
}

/// Проверка конструктора, ожидаем ошибку из за некорректного регулярного выражения байтов
#[test]
#[should_panic(expected = "regular expression is incorrect")]
fn fn_new_t_1() {
    RuleBytes::new(r"(?-u)(?<cstr>[^\x00]+\x00", MatchRequirement::MustBeFound);
}
