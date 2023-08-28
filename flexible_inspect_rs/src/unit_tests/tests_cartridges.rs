use crate::{prelude::*, rules::ModeMatch};

#[test]
fn test_mode_t_0() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [Rule::new(r".+", MatchRequirement::MustBeFound)],
    );
    assert_eq!(
        cartridge.root_rule.0.general_modifiers.mode_match,
        ModeMatch::AllRulesForAtLeastOneMatch
    );
}

#[test]
fn test_mode_t_1() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [Rule::new(r".+", MatchRequirement::MustBeFound)],
    )
    .any_r_for_any_m();
    assert_eq!(
        cartridge.root_rule.0.general_modifiers.mode_match,
        ModeMatch::AtLeastOneRuleForAtLeastOneMatch
    );
}

#[test]
fn test_mode_t_2() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [RuleBytes::new(r".+", MatchRequirement::MustBeFound)],
    );
    assert_eq!(
        cartridge.root_rule.0.general_modifiers.mode_match,
        ModeMatch::AllRulesForAtLeastOneMatch
    );
}

#[test]
fn test_mode_t_5() {
    let cartridge = Cartridge::new(
        404,
        "NOT FOUND",
        [RuleBytes::new(r".+", MatchRequirement::MustBeFound)],
    )
    .any_r_for_any_m();
    assert_eq!(
        cartridge.root_rule.0.general_modifiers.mode_match,
        ModeMatch::AtLeastOneRuleForAtLeastOneMatch
    );
}
