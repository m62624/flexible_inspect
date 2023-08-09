use super::*;
#[test]
fn test_new_t_0() {
    let cartridge: Cartridge<RuleBytes> = PyCartridgeBytes::new(
        0,
        "message_0".into(),
        vec![PyRuleBytes::new(
            r"\w+".into(),
            PyMatchRequeriment::MustBeFound,
        )],
    )
    .into();

    assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![RuleBytes::new(r"\w+", MatchRequirement::MustBeFound)],
        ),
    )
}

#[test]
fn test_mode_match_t_0() {
    let cartridge: Cartridge<RuleBytes> = PyCartridgeBytes::new(
        0,
        "message_0".into(),
        vec![PyRuleBytes::new(
            r"\w+".into(),
            PyMatchRequeriment::MustBeFound,
        )],
    )
    .mode_all_rules_for_at_least_one_match()
    .into();

    assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![RuleBytes::new(r"\w+", MatchRequirement::MustBeFound)],
        )
        .mode_all_rules_for_at_least_one_match(),
    )
}

#[test]
fn test_mode_match_t_1() {
    let cartridge: Cartridge<RuleBytes> = PyCartridgeBytes::new(
        0,
        "message_0".into(),
        vec![PyRuleBytes::new(
            r"\w+".into(),
            PyMatchRequeriment::MustBeFound,
        )],
    )
    .mode_at_least_one_rule_for_all_matches()
    .into();

    assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![RuleBytes::new(r"\w+", MatchRequirement::MustBeFound)],
        )
        .mode_at_least_one_rule_for_all_matches(),
    )
}

#[test]
fn test_mode_match_t_2() {
    let cartridge: Cartridge<RuleBytes> = PyCartridgeBytes::new(
        0,
        "message_0".into(),
        vec![PyRuleBytes::new(
            r"\w+".into(),
            PyMatchRequeriment::MustBeFound,
        )],
    )
    .mode_at_least_one_rule_for_at_least_one_match()
    .into();

    assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![RuleBytes::new(r"\w+", MatchRequirement::MustBeFound)],
        )
        .mode_at_least_one_rule_for_at_least_one_match(),
    )
}
