use super::*;

impl Rule {}

impl SimpleRules {
    /// Constructor for creating simple rules and `regexset`
    /*
    We use `unwrap` instead of `?` because we guarantee that if there are `Rule` that are in this constructor, they have already passed regular expression validity checks
     */
    pub fn new(all_rules: IndexSet<Rule>) -> Self {
        Self {
            regex_set: regex::RegexSet::new(&all_rules).unwrap(),
            all_rules,
        }
    }
}
