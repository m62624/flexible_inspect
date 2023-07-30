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

mod fn_counter_status {
    use super::*;

    #[test]
    fn fn_counter_status_t_0() {
        let mut rule =
            PyRule::new(r"qw".into(), PyMatchRequirement::MustBeFound).counter_is_equal(1);

        assert_eq!(rule.to_rust().get_counter().unwrap(), Counter::Only(1));
    }

    #[test]
    fn fn_counter_status_t_1() {
        let mut rule =
            PyRule::new(r"qw".into(), PyMatchRequirement::MustBeFound).counter_less_than(1);

        assert_eq!(rule.to_rust().get_counter().unwrap(), Counter::LessThan(1));
    }

    #[test]
    fn fn_counter_status_t_2() {
        let mut rule =
            PyRule::new(r"qw".into(), PyMatchRequirement::MustBeFound).counter_more_than(1);

        assert_eq!(rule.to_rust().get_counter().unwrap(), Counter::MoreThan(1));
    }
}

mod fn_extend {

    use super::*;

    #[test]
    fn fn_extend_t_0() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let nested_rules = [
                PyRule::new(r"qw".into(), PyMatchRequirement::MustBeFound),
                PyRule::new(r"qw".into(), PyMatchRequirement::MustBeFound),
            ];
            PyRule::new(r"qw".into(), PyMatchRequirement::MustBeFound)
                .extend(
                    py,
                    PyList::new(py, nested_rules.into_iter().map(|x| x.into_py(py))).into_py(py),
                )
                .unwrap();
        });
    }

    #[test]
    #[should_panic]
    fn fn_extend_t_1() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            PyRule::new(r"qw".into(), PyMatchRequirement::MustBeFound)
                .extend(py, PyList::new(py, [FakeObject.into_py(py)]).into_py(py))
                .unwrap();
        });
    }
}
