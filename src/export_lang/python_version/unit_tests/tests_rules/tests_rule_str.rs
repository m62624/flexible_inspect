use super::*;

mod fn_new {
    use super::*;
    #[test]
    fn fn_new_t_0() {
        PyRule::new(r"x".into(), PyMatchRequirement::MustBeFound);
    }

    #[test]
    fn fn_new_t_1() {
        dbg!(PyRule::new(r"x".into(), PyMatchRequirement::MustNotBeFound));
    }

    #[test]
    #[should_panic]
    fn fn_new_e_0() {
        PyRule::new(r"\x".into(), PyMatchRequirement::MustBeFound);
    }
}

mod matching_modes {
    use super::*;

    #[test]
    fn test_mode_match_t_0() {
        let mut rule = PyRule::new(r"qw".into(), PyMatchRequirement::MustBeFound)
            .mode_all_rules_for_at_least_one_match();

        assert_eq!(
            rule.to_rust().get_mode_match(),
            &ModeMatch::AllRulesForAtLeastOneMatch
        );
    }

    #[test]
    fn test_mode_match_t_1() {
        let mut rule = PyRule::new(r"qw".into(), PyMatchRequirement::MustBeFound)
            .mode_at_least_one_rule_for_at_least_one_match();

        assert_eq!(
            rule.to_rust().get_mode_match(),
            &ModeMatch::AtLeastOneRuleForAtLeastOneMatch
        );
    }

    #[test]
    fn test_mode_match_t_2() {
        let mut rule = PyRule::new(r"qw".into(), PyMatchRequirement::MustBeFound)
            .mode_at_least_one_rule_for_all_matches();

        assert_eq!(
            rule.to_rust().get_mode_match(),
            &ModeMatch::AtLeastOneRuleForAllMatches
        );
    }
}
