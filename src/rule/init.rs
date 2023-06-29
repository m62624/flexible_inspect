use super::*;

mod init_rule {
    use super::*;

    #[pymethods]
    impl Rule {
        #[new]
        pub fn new(pattern: String, requirements: MatchRequirement) -> PyResult<Self> {
            Ok(Self {
                str_raw: Some(RegexRaw::new(pattern)?),
                requirement: Some(requirements),
                subrules: None,
            })
        }
    }
}

mod init_regex_raw {
    use super::*;

    impl RegexRaw {
        pub fn new(pattern: String) -> PyResult<RegexRaw> {
            if regex::Regex::new(&pattern).is_ok() {
                return Ok(RegexRaw::DefaultR(pattern.into_boxed_str()));
            } else if fancy_regex::Regex::new(&pattern).is_ok() {
                return Ok(RegexRaw::FancyR(pattern.into_boxed_str()));
            }
            return Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
                "Expected `Regex` or `FancyRegex`, got `{}`",
                pattern
            )));
        }
    }
}

mod init_subrules {
    use super::*;

    impl Subrules {
        pub fn new(default_rgx_vec: Vec<Rule>, fancy_rgx_vec: Vec<Rule>) -> Self {
            Self {
                default_rgx_set: match !&default_rgx_vec.is_empty() {
                    true => Some(regex::RegexSet::new(&default_rgx_vec).unwrap()),
                    false => None,
                },
                fancy_rgx_vec: match !&fancy_rgx_vec.is_empty() {
                    true => Some(fancy_rgx_vec),
                    false => None,
                },
                default_rgx_vec: match !&default_rgx_vec.is_empty() {
                    true => Some(default_rgx_vec),
                    false => None,
                },
            }
        }
    }
}
