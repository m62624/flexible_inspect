use super::*;
use crate::rules::traits::RuleBase;
use indexmap::IndexSet;

/// Проверяем, что сортировка работает корректно
#[test]
fn fn_extend_t_0() {
    let rule = Rule::new(r"some text (?=\.)", MatchRequirement::MustBeFound).extend([
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
    let complex_rules = IndexSet::from([
        Rule::new(r"\d+(?=\w+)", MatchRequirement::MustBeFound),
        Rule::new(r"\d+(?=\w+)[1-9]", MatchRequirement::MustBeFound),
    ]);

    assert_eq!(
        rule.content_unchecked()
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
            .subrules
            .as_ref()
            .unwrap()
            .complex_rules
            .as_ref()
            .unwrap(),
        &complex_rules
    );
}

/// Проверяем на значение None если коллекция пустая
#[test]
fn fn_extend_t_1() {
    let rule = Rule::new(r"some text (?=\.)", MatchRequirement::MustBeFound).extend([]);
    assert_eq!(rule.content_unchecked().subrules, None);
}
