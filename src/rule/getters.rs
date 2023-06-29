use super::*;

mod for_rule {
    use super::*;

    impl Rule {
        pub fn get_requirement(&self) -> PyResult<&MatchRequirement> {
            if let Some(value) = &self.requirement {
                return Ok(value);
            } else {
                return Err(absence_error());
            }
        }
        pub fn get_str_raw(&self) -> PyResult<&RegexRaw> {
            if let Some(value) = &self.str_raw {
                return Ok(value);
            } else {
                return Err(absence_error());
            }
        }
        pub fn get_op_subrules(&self) -> &Option<Subrules> {
            &self.subrules
        }
        pub fn get_selected_rules<'a>(regex_set: &'a regex::RegexSet, text: &str) -> Vec<usize> {
            regex_set
                .matches(text)
                .iter()
                .map(|id| id)
                .collect::<Vec<_>>()
        }
    }

    fn absence_error() -> PyErr {
        PyErr::new::<exceptions::PyValueError, _>(format!(
            "* If you saved `Rule` in a variable, but used `extend` afterwards on the variable itself:
    
           x = Rule(\"X\")
           x.extend(Rule(\"Y\"))
           
           * Please use this syntax:
           
           x = Rule(\"X\").extend(Rule(\"Y\"))
           * or 
           x = Rule(\"X\")
           x = x.extend(Rule(\"Y\"))"
        ))
    }

    impl AsRef<str> for Rule {
        fn as_ref(&self) -> &str {
            self.get_str_raw().unwrap().as_ref()
        }
    }
    mod unchacked_getters {
        use super::*;
        impl Rule {
            pub fn unchacked_get_rgx_set(&self) -> &Option<regex::RegexSet> {
                self.get_op_subrules()
                    .as_ref()
                    .unwrap()
                    .get_default_rgx_set()
            }
            pub fn unchacked_get_rgx_vec(&self) -> &Vec<Rule> {
                self.get_op_subrules()
                    .as_ref()
                    .unwrap()
                    .default_rgx_vec
                    .as_ref()
                    .unwrap()
            }
        }
    }
}

mod for_regex_raw {
    use super::*;

    impl AsRef<str> for RegexRaw {
        fn as_ref(&self) -> &str {
            match self {
                RegexRaw::DefaultR(value) => value,
                RegexRaw::FancyR(value) => value,
            }
        }
    }
}

mod for_subrules {
    use super::*;

    impl Subrules {
        pub fn get_default_rgx_vec<'a>(&self) -> &Option<Vec<Rule>> {
            &self.default_rgx_vec
        }
        pub fn get_fancy_rgx_vec<'a>(&self) -> &Option<Vec<Rule>> {
            &self.fancy_rgx_vec
        }
        pub fn get_default_rgx_set<'a>(&self) -> &Option<regex::RegexSet> {
            &self.default_rgx_set
        }
    }
}
