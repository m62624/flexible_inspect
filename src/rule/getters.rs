use super::*;
use pyo3::exceptions;
impl Rule {
    fn error_take() -> PyErr {
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
    pub fn get_op_rule_raw(&self) -> &Option<RuleType> {
        &self.rule_raw
    }
    pub fn get_op_requirement(&self) -> &Option<MatchRequirement> {
        &self.requirement
    }
    pub fn get_op_subrules(&self) -> &Option<Vec<Rule>> {
        &self.subrules
    }
    pub fn get_rule_raw(&self) -> PyResult<&RuleType> {
        if let Some(rule_raw) = &self.rule_raw {
            Ok(rule_raw)
        } else {
            Err(Rule::error_take())
        }
    }
    pub fn get_requirement(&self) -> PyResult<&MatchRequirement> {
        if let Some(requirement) = &self.requirement {
            Ok(requirement)
        } else {
            Err(Rule::error_take())
        }
    }
}

impl RuleType {
    fn get_str(&self) -> &Box<str> {
        match self {
            RuleType::DefaultRegex(str_value) => str_value,
            RuleType::FancyRegex(str_value) => str_value,
        }
    }
}
