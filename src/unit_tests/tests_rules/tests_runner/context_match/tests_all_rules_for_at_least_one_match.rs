use crate::core::rules::{self, next::NextStep};
use crate::prelude::*;

#[test]
fn test_runner_t_0() {
    let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"1", MatchRequirement::MustBeFound),
        Rule::new(r"\d{4}", MatchRequirement::MustBeFound),
    ])
    .mode_all_rules_for_at_least_one_match()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Finish
    );
}

#[test]
fn test_runner_t_1() {
    let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"A", MatchRequirement::MustBeFound),
        Rule::new(r"\d{3}", MatchRequirement::MustBeFound),
    ])
    .mode_all_rules_for_at_least_one_match()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Error(None)
    );
}

#[test]
fn test_runner_t_2() {
    let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"A", MatchRequirement::MustNotBeFound),
        Rule::new(r"\d(?=\d+)", MatchRequirement::MustBeFound),
    ])
    .mode_all_rules_for_at_least_one_match()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Finish
    );
}

#[test]
fn test_runner_t_3() {
    let text = "[ [2] [1] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"A", MatchRequirement::MustNotBeFound),
        Rule::new(r"\d(?=\d+)", MatchRequirement::MustBeFound),
    ])
    .mode_all_rules_for_at_least_one_match()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Error(None)
    );
}
