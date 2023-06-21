use super::*;
impl Rule {
    pub fn get_inner(&self) -> &Option<(String, regex_types::RGX)> {
        &self.inner
    }
    pub fn get_requirement(&self) -> &Option<MatchRequirement> {
        &self.requirement
    }
    pub fn get_rules_for_the_rule(&self) -> &Option<Vec<Rule>> {
        &self.rules_for_the_rule
    }
    pub fn get_regex_set(&self) -> &Option<regex::RegexSet> {
        &self.regex_set
    }
}
