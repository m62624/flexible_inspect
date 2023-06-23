use super::*;
impl Rule {
    pub fn get_inner(&self) -> PyResult<&(Box<str>, regex_types::RGX)> {
        if let Some(value) = &self.rule_raw {
            return Ok(value);
        }
        return Err(error_take());
    }
    pub fn get_requirement(&self) -> PyResult<&MatchRequirement> {
        if let Some(value) = &self.requirement {
            return Ok(value);
        }
        return Err(error_take());
    }
    pub fn get_rules_for_the_rule(&self) -> &Option<Vec<Rule>> {
        &self.rules_for_the_rule
    }

    pub fn is_exist_subrules(&self) -> bool {
        self.rules_for_the_rule.is_some()
    }

    // pub fn get_regex_set(&self) -> &Option<regex::RegexSet> {
    //     &self.regex_set
    // }
}

fn error_take() -> PyErr {
    PyErr::new::<exceptions::PyValueError, _>(format!(
        "* If you saved `Rule` in a variable, but used `extend` afterwards on the variable itself:

        x = Rule(\"x\")
        x.extend(Rule(\"Y\"))
        
        * Please use this syntax:
        
        x = Rule(\"x\").extend(Rule(\"y\"))
        * or 
        x = Rule(\"x\")
        x = x.extend(Rule(\"y\"))"
    ))
}
