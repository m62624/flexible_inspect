use super::*;

#[test]
fn fn_runner_t_0() {
    let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"A", MatchRequirement::MustNotBeFound),
        Rule::new(r"B", MatchRequirement::MustBeFound),
        Rule::new(r"C", MatchRequirement::MustBeFound),
    ])
    .any_r_for_all_m()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, Rule::find_captures(&rule, &text)),
        NextStep::Finish
    );
}

#[test]
fn fn_runner_t_1() {
    let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"A", MatchRequirement::MustBeFound),
        Rule::new(r"B", MatchRequirement::MustBeFound),
        Rule::new(r"C", MatchRequirement::MustBeFound),
    ])
    .any_r_for_all_m()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, Rule::find_captures(&rule, &text)),
        NextStep::Error(None)
    );
}

#[test]
fn fn_runner_t_2() {
    let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"B", MatchRequirement::MustBeFound),
        Rule::new(r"\d(?=\d+)", MatchRequirement::MustBeFound),
    ])
    .any_r_for_all_m()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, Rule::find_captures(&rule, &text)),
        NextStep::Finish
    );
}

#[test]
fn fn_runner_t_3() {
    let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"B", MatchRequirement::MustBeFound),
        Rule::new(r"\d(?=C)", MatchRequirement::MustBeFound),
    ])
    .any_r_for_all_m()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, Rule::find_captures(&rule, &text)),
        NextStep::Error(None)
    );
}

#[test]
fn fn_runner_t_4() {
    let text = "[ [123] [123] [1234] ] [ [123456789] [12345678] ]";
    let rule: Rule = Rule::new(r".+", MatchRequirement::MustBeFound).extend([Rule::new(
        r"\[[^\[\]]+\]",
        MatchRequirement::MustBeFound,
    )
    .extend([
        Rule::new(r"\d+", MatchRequirement::MustBeFound),
        Rule::new(r"B", MatchRequirement::MustBeFound),
        Rule::new(r"C", MatchRequirement::MustBeFound),
    ])
    .any_r_for_all_m()]);
    assert_eq!(
        rules::runner::run::<Rule, &str>(&rule, Rule::find_captures(&rule, &text)),
        NextStep::Finish
    );
}
