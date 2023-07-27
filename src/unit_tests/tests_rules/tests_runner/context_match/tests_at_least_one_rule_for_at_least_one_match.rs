use crate::core::rules::{self, next::NextStep};
use crate::prelude::*;

#[test]
fn test_runner_t_0() {
    let text = "[ [123] [12345678911] [12345678911] ] [12345678911] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"\d{10}", MatchRequirement::MustNotBeFound),
        Rule::new(r"x", MatchRequirement::MustBeFound),
    ])
    .mode_at_least_one_rule_for_at_least_one_match()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Finish
    );
}

#[test]
fn test_runner_t_1() {
    let text = "[ [12345678911] [12345678911] [12345678911] ] [12345678911] [12345678911] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"12345678911", MatchRequirement::MustNotBeFound),
        Rule::new(r"x", MatchRequirement::MustBeFound),
    ])
    .mode_at_least_one_rule_for_at_least_one_match()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Error(None)
    );
}

#[test]
fn test_runner_t_2() {
    let text = "[ [123] [12345678911] [A] [12345678911] ] [ [12345678911] [1234567811] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"A(?=\])", MatchRequirement::MustBeFound),
        Rule::new(r"x", MatchRequirement::MustBeFound),
    ])
    .mode_at_least_one_rule_for_at_least_one_match()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Finish
    );
}

#[test]
fn test_runner_t_3() {
    let text = "[ [123] [12345678911] [A] [12345678911] ] [ [12345678911] [1234567811] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"A(?=\]\])", MatchRequirement::MustBeFound),
        Rule::new(r"x", MatchRequirement::MustBeFound),
    ])
    .mode_at_least_one_rule_for_at_least_one_match()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Error(None)
    );
}

#[test]
fn test_runner_t_4() {
    let text = "[ [A] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"A", MatchRequirement::MustNotBeFound),
        Rule::new(r"x", MatchRequirement::MustBeFound),
    ])
    .mode_at_least_one_rule_for_at_least_one_match()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Error(None)
    );
}

#[test]
fn test_runner_t_5() {
    let text = "[ [A] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"A", MatchRequirement::MustNotBeFound),
        Rule::new(r"x", MatchRequirement::MustBeFound),
    ])
    .mode_at_least_one_rule_for_at_least_one_match()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Error(None)
    );
}
