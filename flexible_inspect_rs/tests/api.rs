use flexible_inspect_rs::prelude::*;

mod api_rules {
    use super::*;

    #[test]
    fn rule_str() {
        let rule1 = Rule::new(r"\d+", MatchRequirement::MustBeFound);
        let rule2 = Rule::new(r"\s+", MatchRequirement::MustNotBeFound);


    }
}
