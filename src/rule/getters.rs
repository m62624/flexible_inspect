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
    pub fn get_op_str_raw(&self) -> &Option<RegexRaw> {
        &self.str_raw
    }
    pub fn get_op_requirement(&self) -> &Option<MatchRequirement> {
        &self.requirement
    }
    pub fn get_op_subrules(&self) -> &Option<Vec<Rule>> {
        &self.subrules
    }
    pub fn get_str_raw(&self) -> PyResult<&RegexRaw> {
        if let Some(rule_raw) = &self.str_raw {
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

impl RegexRaw {
    pub fn get_str(&self) -> &Box<str> {
        match self {
            RegexRaw::DefaultR(value) => value,
            RegexRaw::FancyR(value) => value,
        }
    }
}
