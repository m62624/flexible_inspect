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
        pub fn get_selected_rules<'a>(
            regex_set: &'a Option<regex::RegexSet>,
            default_rules: &'a Option<Vec<Rule>>,
            text: &str,
        ) -> Option<Vec<&'a Rule>> {
            if let Some(rgxst) = regex_set {
                return Some(
                    rgxst
                        .matches(text)
                        .iter()
                        .map(|id| &default_rules.as_ref().unwrap()[id])
                        .collect::<Vec<_>>(),
                );
            }
            None
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
