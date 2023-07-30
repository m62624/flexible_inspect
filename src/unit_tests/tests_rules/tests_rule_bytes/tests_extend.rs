use super::*;


/// Проверяем на корректное расширение правила байтов
#[test]
pub fn fn_extend_t_0() {
    RuleBytes::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound).extend([
        RuleBytes::new(
            r"(?-u)\x7b\xa9(?:[\x80-\xfe]|[\x40-\xff].)(?u:(.*))",
            MatchRequirement::MustBeFound,
        ),
        RuleBytes::new(r"\x00", MatchRequirement::MustNotBeFound),
    ]);
}

/// Проверяем на значение None если коллекция пустая
#[test]
fn fn_extend_t_1() {
    let rule =
        RuleBytes::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound).extend([]);
    assert_eq!(rule.content_unchecked().subrules_bytes, None);
}
