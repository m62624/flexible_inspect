use super::*;

#[test]
fn test_new_t_0() {
    let rule: Rule = PyRule::new(r"\w+".into(), PyMatchRequeriment::MustBeFound).into();

    assert_eq!(rule, Rule::new(r"\w+", MatchRequirement::MustBeFound));
}

#[test]
fn test_new_t_1() {
    let rule: Rule = PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound).into();

    assert_eq!(rule, Rule::new(r"\w+", MatchRequirement::MustNotBeFound));
}

#[test]
fn test_extend_t_0() {
    let rule: Rule = PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .extend(vec![
            PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound).into(),
            PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound).into(),
            PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound).into(),
        ])
        .into();

    assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).extend(vec![
            Rule::new(r"\w+", MatchRequirement::MustNotBeFound),
            Rule::new(r"\w+", MatchRequirement::MustNotBeFound),
            Rule::new(r"\w+", MatchRequirement::MustNotBeFound),
        ])
    );
}

#[test]
fn test_mode_counter_t_0() {
    let rule: Rule = PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .counter_is_equal(1)
        .unwrap()
        .into();

    assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).counter_is_equal(1)
    );
}

#[test]
fn test_mode_counter_t_1() {
    let rule: Rule = PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .counter_less_than(1)
        .into();

    assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).counter_less_than(1)
    );
}

#[test]
fn test_mode_counter_t_2() {
    let rule: Rule = PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .counter_more_than(1)
        .into();

    assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).counter_more_than(1)
    );
}

#[test]
fn test_mode_match_t_0() {
    let rule: Rule = PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .all_r_for_any_m()
        .into();

    assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).all_r_for_any_m()
    );
}

#[test]
fn test_mode_match_t_1() {
    let rule: Rule = PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .any_r_for_all_m()
        .into();

    assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).any_r_for_all_m()
    );
}

#[test]
fn test_mode_match_t_2() {
    let rule: Rule = PyRule::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .any_r_for_any_m()
        .into();

    assert_eq!(
        rule,
        Rule::new(r"\w+", MatchRequirement::MustNotBeFound).any_r_for_any_m()
    );
}
