use super::*;

impl Rule {
    pub fn get_content(&self) -> PyResult<&TakeRuleForExtend> {
        if let Some(content) = &self.content {
            return Ok(content);
        }
        Err(Self::option_error())
    }

    pub fn get_content_mut(&mut self) -> PyResult<&mut TakeRuleForExtend> {
        if let Some(content) = &mut self.content {
            return Ok(content);
        }
        Err(Self::option_error())
    }

    pub fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize> {
        regex_set.matches(text).iter().collect::<Vec<_>>()
    }

    fn option_error() -> PyErr {
        PyErr::new::<exceptions::PyTypeError, _>(
            "If you saved `Rule` in a variable, but used `extend` afterwards on the variable itself:
    
            x = Rule(\"X\")
            x.extend(Rule(\"Y\"))
            
            * Please use this syntax:
            
            x = Rule(\"X\").extend(Rule(\"Y\"))
            * or 
            x = Rule(\"X\")
            x = x.extend(Rule(\"Y\"))",
        )
    }
}

impl TakeRuleForExtend {
    // pub fn get_str_with_type(&self) -> &RegexRaw {
    //     &self.str_with_type
    // }
    // pub fn get_requirement(&self) -> &MatchRequirement {
    //     &self.requirement
    // }
    // pub fn get_subrules(&self) -> Option<&Subrules> {
    //     self.subrules.as_ref()
    // }
}
