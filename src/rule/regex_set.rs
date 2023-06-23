use super::*;
impl Rule {
    pub fn init_root_regex_set(rules: &Vec<(rule::Rule, usize)>) -> regex::RegexSet {
        regex::RegexSet::new(
            rules
                .iter()
                .filter_map(|rule| {
                    if let Some(value) = &rule.0.rule_raw {
                        if let regex_types::RGX::Default = value.1 {
                            return Some(&value.0);
                        }
                    }
                    None
                })
                .collect::<Vec<&Box<str>>>(),
        )
        .unwrap()
    }

    pub fn make_sub_regex_set(subrules: &Option<Vec<Rule>>) -> Option<regex::RegexSet> {
        if let Some(subrules) = subrules {
            return Some(
                regex::RegexSet::new(
                    subrules
                        .iter()
                        .filter_map(|value| {
                            if let Some(value) = &value.rule_raw {
                                if let regex_types::RGX::Default = value.1 {
                                    return Some(&value.0);
                                }
                            }
                            None
                        })
                        .collect::<Vec<&Box<str>>>(),
                )
                .unwrap(),
            );
        }
        None
    }
}
