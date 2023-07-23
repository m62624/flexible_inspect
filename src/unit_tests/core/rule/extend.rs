use indexmap::IndexSet;

use super::*;

/// Проверяем, что успешно работает расширение, когда корень регулярного выражения для &str и добавляем регулярное выражение для &str
#[test]
pub fn extend_t_0() {
    Rule::new(r"\d", MatchRequirement::MustBeFound)
        .extend([Rule::new(r"\d", MatchRequirement::MustBeFound)]);
}

/// Проверяем, что выходит ошибка при добавлении регулярного выражения для &[u8] байтов, когда корень регулярного выражения для &str
#[test]
#[should_panic(
    expected = "only regular expressions for `&str` strings can be added to a regular expression for `&str` strings, not for &[u8] bytes"
)]
pub fn extend_t_1() {
    Rule::new(r"\d", MatchRequirement::MustBeFound).extend([Rule::new(
        r"(?-u)(?<cstr>[^\x00]+)\x00",
        MatchRequirement::MustBeFound,
    )
    .for_bytes()]);
}

/// Проверяем, что успешно работает расширение, когда корень регулярного выражения для &[u8] байтов и добавляем регулярное выражение для &[u8] байтов
#[test]
pub fn extend_t_2() {
    Rule::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound)
        .for_bytes()
        .extend([
            Rule::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound).for_bytes(),
        ]);
}

/// Проверяем, что выходит ошибка при добавлении регулярного выражения для &str, когда корень регулярного выражения для &[u8] байтов
#[test]
#[should_panic(
    expected = "only regular expressions for &[u8] bytes can be added to a regular expression for &[u8] bytes, not for `&str` strings"
)]
pub fn extend_t_3() {
    Rule::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound)
        .for_bytes()
        .extend([Rule::new(r"\d", MatchRequirement::MustBeFound)]);
}

/// Проверяем, что сортировка работает корректно
#[test]
pub fn extend_t_4() {
    let mut rule = Rule::new(r"some text (?=\.)", MatchRequirement::MustBeFound).extend([
        Rule::new(r"\d", MatchRequirement::MustBeFound),
        Rule::new(r"\d+(?=\w+)", MatchRequirement::MustBeFound),
        Rule::new(r"\s", MatchRequirement::MustBeFound),
        Rule::new(r"123", MatchRequirement::MustBeFound),
        Rule::new(r"\d+(?=\w+)[1-9]", MatchRequirement::MustBeFound),
    ]);

    let simple_rules: IndexSet<Rule> = IndexSet::from([
        Rule::new(r"\d", MatchRequirement::MustBeFound),
        Rule::new(r"\s", MatchRequirement::MustBeFound),
        Rule::new(r"123", MatchRequirement::MustBeFound),
    ]);
    let complex_rules = vec![
        Rule::new(r"\d+(?=\w+)", MatchRequirement::MustBeFound),
        Rule::new(r"\d+(?=\w+)[1-9]", MatchRequirement::MustBeFound),
    ];

    assert_eq!(
        rule.content_unchecked()
            .mutable_modifiers
            .subrules
            .as_ref()
            .unwrap()
            .simple_rules
            .as_ref()
            .unwrap()
            .all_rules,
        simple_rules
    );
    assert_eq!(
        rule.content_unchecked()
            .mutable_modifiers
            .subrules
            .as_ref()
            .unwrap()
            .complex_rules
            .as_ref()
            .unwrap(),
        &complex_rules
    );
}
