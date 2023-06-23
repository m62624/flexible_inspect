use super::*;
impl Rule {
    pub fn init_root_regex_set(rules: &Vec<(rule::Rule, usize)>, text: &str) -> Vec<usize> {
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
        .matches(text)
        .iter()
        .collect::<Vec<_>>()
    }

    pub fn make_sub_regex_set(subrules: &Option<Vec<Rule>>, text: &str) -> Option<Vec<usize>> {
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
                .unwrap()
                .matches(text)
                .iter()
                .collect::<Vec<_>>(),
            );
        }
        None
    }
}

#[test]
fn show_root_rule() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        dbg!(rule::Rule::make_sub_regex_set(
            &Some(vec![
                Rule::new(
                    "root 5 ^a(?>bc|b)c$".to_string(),
                    MatchRequirement::MustBeFound
                )
                .unwrap(),
                Rule::new("root - AV".to_string(), MatchRequirement::MustBeFound).unwrap(),
                Rule::new("root 2".to_string(), MatchRequirement::MustBeFound).unwrap(),
                Rule::new("root 3".to_string(), MatchRequirement::MustBeFound).unwrap(),
                Rule::new("root 4".to_string(), MatchRequirement::MustBeFound).unwrap(),
            ]),
            "root - AV root 2 root"
        ));
    });
}
