use super::*;
use crate::core::rules::{self, traits::RuleBase};

#[test]
fn test_runner_t_0() {
    let text = "text text [p] [123] [456] [alq]";
    let rule: Rule = Rule::new(r"\[.+\]", MatchRequirement::MustBeFound)
        .extend([Rule::new(r"\d+", MatchRequirement::MustBeFound)]);

    rules::runner::run::<Rule, &str>(&rule, text);
}

#[test]
fn test_runner_t_1() {
    let text = b"text text [p] [123] [456] [alq]";
    let rule: RuleBytes = RuleBytes::new(r"\[[^\[\]+]\]", MatchRequirement::MustBeFound)
        .extend([RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)]);

    rules::runner::run::<RuleBytes, &[u8]>(&rule, text);
}
