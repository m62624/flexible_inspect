use std::collections::HashMap;

use super::*;
use crate::core::rules::{self, next::NextStep};

#[test]
fn test_runner_t_0() {
    let text = "[ [12346] [132] [1234] ] [ [123456789] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"\d+", MatchRequirement::MustBeFound),
        Rule::new(r"\d{4}", MatchRequirement::MustBeFound),
    ])]);

    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Error(None)
    );
}

#[test]
fn test_runner_t_1() {
    let text = "[ [12346] [132] [1234] ] [ [123456789] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"\d+", MatchRequirement::MustBeFound),
        Rule::new(r"\d{3}", MatchRequirement::MustBeFound),
    ])]);

    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Finish
    );
}

#[test]
fn test_runner_t_2() {
    let text = "[ [1111] [1111] [1111] ] [ [1111] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"\d+", MatchRequirement::MustBeFound),
        Rule::new(r"\d{3}", MatchRequirement::MustNotBeFound),
    ])]);

    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Error(Some(HashMap::from([("main_capture".into(), "111".into())])))
    );
}

#[test]
fn test_runner_t_3() {
    let text = "[ [12346] [132] [1234] ] [ [123456789] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"\d+", MatchRequirement::MustBeFound),
        Rule::new(r"\d(?=A)", MatchRequirement::MustBeFound),
    ])]);

    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Error(None)
    );
}

#[test]
fn test_runner_t_4() {
    let text = "[ [12346] [132] [1234] ] [ [123456789] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"\d+", MatchRequirement::MustBeFound),
        Rule::new(r"\d(?=\d+)", MatchRequirement::MustBeFound),
    ])]);

    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Finish
    );
}

#[test]
fn test_runner_t_5() {
    let text = "[ [12346] [132] [1234] ] [ [123456789] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"\d+", MatchRequirement::MustBeFound),
        Rule::new(r"ABC", MatchRequirement::MustNotBeFound),
    ])]);

    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, text),
        NextStep::Finish
    );
}
