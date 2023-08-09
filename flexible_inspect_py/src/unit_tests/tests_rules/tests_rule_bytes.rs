use super::*;

#[test]
fn test_new() {
    let rule: RuleBytes = PyRuleBytes::new(r"\w+".into(), PyMatchRequeriment::MustBeFound).into();

    assert_eq!(rule, RuleBytes::new(r"\w+", MatchRequirement::MustBeFound));
}

#[test]
fn test_new_t_1() {
    let rule: RuleBytes = PyRuleBytes::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound).into();

    assert_eq!(rule, RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound));
}


#[test]
fn test_extend() {
    let rule: RuleBytes = PyRuleBytes::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .extend(vec![
            PyRuleBytes::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound).into(),
            PyRuleBytes::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound).into(),
            PyRuleBytes::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound).into(),
        ])
        .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound).extend(vec![
            RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound),
            RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound),
            RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound),
        ])
    );
}

#[test]
fn test_mode_match_t_0() {
    let rule: RuleBytes = PyRuleBytes::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .mode_all_rules_for_at_least_one_match()
        .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound).mode_all_rules_for_at_least_one_match()
    );
}

#[test]
fn test_mode_match_t_1() {
    let rule: RuleBytes = PyRuleBytes::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .mode_at_least_one_rule_for_all_matches()
        .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound)
            .mode_at_least_one_rule_for_all_matches()
    );
}

#[test]
fn test_mode_match_t_2() {
    let rule: RuleBytes = PyRuleBytes::new(r"\w+".into(), PyMatchRequeriment::MustNotBeFound)
        .mode_at_least_one_rule_for_at_least_one_match()
        .into();

    assert_eq!(
        rule,
        RuleBytes::new(r"\w+", MatchRequirement::MustNotBeFound)
            .mode_at_least_one_rule_for_at_least_one_match()
    );
}


