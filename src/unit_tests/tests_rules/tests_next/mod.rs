mod tests_counter_status;

use super::*;
use crate::core::rules::next::NextStep;
use crate::core::rules::traits::RuleBytesExtendBase;
use crate::core::rules::traits::RuleExtendBase;
use crate::core::DEFAULT_CAPTURE;
use std::collections::HashMap;

/// Проверка Next для двух типов `Rule`, технический хватает только тест одного типа,
/// но эти главный корень для остальных модификтаровов, поэтому проверяем оба типа
mod core_rule_tests {
    use super::*;

    mod rule_without_modifiers {
        use super::*;

        /// MustBeFound
        /// Captures - TRUE
        /// Subrules - NONE
        #[test]
        fn next_or_finish_or_error_t_0() {
            let rule = Rule::new(r"\d+", MatchRequirement::MustBeFound);
            let mut captures = Rule::find_captures(&rule, "123412");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Finish
            );
        }

        /// MustBeFound
        /// Captures - TRUE
        /// Subrules - TRUE
        #[test]
        fn next_or_finish_or_error_t_1() {
            let rule = Rule::new(r"\d+", MatchRequirement::MustBeFound)
                .extend([Rule::new(r"\d", MatchRequirement::MustBeFound)]);
            let mut captures = Rule::find_captures(&rule, "123412");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Go
            );
        }

        /// MustBeFound
        /// Captures - FALSE
        /// Subrules - TRUE
        #[test]
        fn next_or_finish_or_error_t_2() {
            let rule = Rule::new(r"\d+", MatchRequirement::MustBeFound)
                .extend([Rule::new(r"\d", MatchRequirement::MustBeFound)]);
            let mut captures = Rule::find_captures(&rule, "abc");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Error(None)
            );
        }

        /// MustBeFound
        /// Captures - FALSE
        /// Subrules - FALSE
        #[test]
        fn next_or_finish_or_error_t_3() {
            let rule = Rule::new(r"\d+", MatchRequirement::MustBeFound);
            let mut captures = Rule::find_captures(&rule, "abc");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Error(None)
            );
        }

        /// MustNotBeFound
        /// Captures - TRUE
        /// Subrules - NONE
        #[test]
        fn next_or_finish_or_error_t_4() {
            let rule = Rule::new(r"\d+", MatchRequirement::MustNotBeFound);
            let mut captures = Rule::find_captures(&rule, "123412");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Error(Some(HashMap::from([(
                    DEFAULT_CAPTURE.into(),
                    "123412".into()
                )])))
            );
        }

        /// MustNotBeFound
        /// Captures - TRUE
        /// Subrules - TRUE
        #[test]
        fn next_or_finish_or_error_t_5() {
            let rule = Rule::new(r"\d+", MatchRequirement::MustNotBeFound)
                .extend([Rule::new(r"\d", MatchRequirement::MustBeFound)]);
            let mut captures = Rule::find_captures(&rule, "123412");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Go
            );
        }

        /// MustNotBeFound
        /// Captures - FALSE
        /// Subrules - TRUE
        #[test]
        fn next_or_finish_or_error_t_6() {
            let rule = Rule::new(r"\d+", MatchRequirement::MustNotBeFound)
                .extend([Rule::new(r"\d", MatchRequirement::MustBeFound)]);
            let mut captures = Rule::find_captures(&rule, "abc");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Finish
            );
        }

        /// MustNotBeFound
        /// Captures - FALSE
        /// Subrules - FALSE
        #[test]
        fn next_or_finish_or_error_t_7() {
            let rule = Rule::new(r"\d+", MatchRequirement::MustNotBeFound);
            let mut captures = Rule::find_captures(&rule, "abc");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Finish
            );
        }
    }

    mod rule_bytes_without_modifiers {
        use super::*;
        /// MustBeFound
        /// Captures - TRUE
        /// Subrules - NONE
        #[test]
        fn next_or_finish_or_error_t_0() {
            let rule = RuleBytes::new(r"\d+", MatchRequirement::MustBeFound);
            let mut captures = RuleBytes::find_captures(&rule, b"123412");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Finish
            );
        }

        /// MustBeFound
        /// Captures - TRUE
        /// Subrules - TRUE
        #[test]
        fn next_or_finish_or_error_t_1() {
            let rule = RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)
                .extend([RuleBytes::new(r"\d", MatchRequirement::MustBeFound)]);
            let mut captures = RuleBytes::find_captures(&rule, b"123412");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Go
            );
        }

        /// MustBeFound
        /// Captures - FALSE
        /// Subrules - TRUE
        #[test]
        fn next_or_finish_or_error_t_2() {
            let rule = RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)
                .extend([RuleBytes::new(r"\d", MatchRequirement::MustBeFound)]);
            let mut captures = RuleBytes::find_captures(&rule, b"abc");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Error(None)
            );
        }

        /// MustBeFound
        /// Captures - FALSE
        /// Subrules - FALSE
        #[test]
        fn next_or_finish_or_error_t_3() {
            let rule = RuleBytes::new(r"\d+", MatchRequirement::MustBeFound);
            let mut captures = RuleBytes::find_captures(&rule, b"abc");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Error(None)
            );
        }

        /// MustNotBeFound
        /// Captures - TRUE
        /// Subrules - NONE
        #[test]
        fn next_or_finish_or_error_t_4() {
            let rule = RuleBytes::new(r"\d+", MatchRequirement::MustNotBeFound);
            let mut captures = RuleBytes::find_captures(&rule, b"123412");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Error(Some(HashMap::from([(
                    DEFAULT_CAPTURE.into(),
                    "[49,50,51,52,49,50]".into()
                )])))
            );
        }

        /// MustNotBeFound
        /// Captures - TRUE
        /// Subrules - TRUE
        #[test]
        fn next_or_finish_or_error_t_5() {
            let rule = RuleBytes::new(r"\d+", MatchRequirement::MustNotBeFound)
                .extend([RuleBytes::new(r"\d", MatchRequirement::MustBeFound)]);
            let mut captures = RuleBytes::find_captures(&rule, b"123412");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Go
            );
        }

        /// MustNotBeFound
        /// Captures - FALSE
        /// Subrules - TRUE
        #[test]
        fn next_or_finish_or_error_t_6() {
            let rule = RuleBytes::new(r"\d+", MatchRequirement::MustNotBeFound)
                .extend([RuleBytes::new(r"\d", MatchRequirement::MustBeFound)]);
            let mut captures = RuleBytes::find_captures(&rule, b"abc");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Finish
            );
        }

        /// MustNotBeFound
        /// Captures - FALSE
        /// Subrules - FALSE
        #[test]
        fn next_or_finish_or_error_t_7() {
            let rule = RuleBytes::new(r"\d+", MatchRequirement::MustNotBeFound);
            let mut captures = RuleBytes::find_captures(&rule, b"abc");
            assert_eq!(
                NextStep::next_or_finish_or_error(rule, &mut captures),
                NextStep::Finish
            );
        }
    }
}
