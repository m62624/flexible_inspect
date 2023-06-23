use super::*;
impl Rule {
    pub fn init_root_regex_set<'a>(rules: &'a Vec<(rule::Rule, usize)>) -> Vec<&'a Box<str>> {
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
            .collect()
    }

    pub fn make_sub_regex_set<'a>(subrules: &'a Option<Vec<Rule>>) -> Option<Vec<&'a Box<str>>> {
        if let Some(subrules) = subrules {
            return Some(
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
                    .collect(),
            );
        }
        None
    }
}
