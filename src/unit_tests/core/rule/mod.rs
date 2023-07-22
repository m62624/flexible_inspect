use super::*;

/// Проверка конструктора `Rule`
mod fn_new {
    use super::*;
    use crate::core::rule::RegexRaw;

    /// Должны определить, что это `DefaultRegex`
    #[test]
    fn new_t_0() {
        let rule = Rule::new(r"\d", MatchRequirement::MustBeFound);
        assert_eq!(
            &rule.content_unchecked().str_with_type,
            &RegexRaw::DefaultRegex(String::from(r"\d").into_boxed_str())
        );
    }

    /// Должны определить, что это `FancyRegex`
    #[test]
    fn new_t_1() {
        let rule = Rule::new(r"\d(?=\d)", MatchRequirement::MustBeFound);
        assert_eq!(
            &rule.content_unchecked().str_with_type,
            &RegexRaw::FancyRegex(String::from(r"\d(?=\d)").into_boxed_str())
        );
    }

    #[test]
    #[should_panic(expected = "regular expression is incorrect")]
    fn new_e_0() {
        Rule::new(r"\xawq", MatchRequirement::MustNotBeFound);
    }
}
