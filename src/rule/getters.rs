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
