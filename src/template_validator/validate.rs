use super::*;

pub fn action_on_the_matches(matches: &Vec<&str>, rule_contrainer: &rule::Rule) -> PyResult<bool> {
    match rule_contrainer.get_requirement()? {
        rule::MatchRequirement::MustBeFound => Ok(!matches.is_empty()),
        rule::MatchRequirement::MustNotBefound => {
            if rule_contrainer.is_exist_subrules() {
                Ok(!matches.is_empty())
            } else {
                Ok(matches.is_empty())
            }
        }
    }
}
