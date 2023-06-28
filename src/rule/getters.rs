use super::*;

impl Rule {
    pub fn get_match_requirement(&self) -> PyResult<&MatchRequirement> {
        if let Some(value) = &self.requirement {
            return Ok(value);
        } else {
            return Err(Rule::absence_error());
        }
    }
    pub fn get_str_raw(&self) -> PyResult<&RegexRaw> {
        if let Some(value) = &self.str_raw {
            return Ok(value);
        } else {
            return Err(Rule::absence_error());
        }
    }
    pub fn get_op_subrules(&self) -> &Option<Subrules> {
        &self.subrules
    }
}

impl RegexRaw {
    pub fn get_str(&self) -> &str {
        match self {
            RegexRaw::DefaultR(value) => &value,
            RegexRaw::FancyR(value) => &value,
        }
    }
}

impl Subrules {
    pub fn get_default_r_vec(&self) -> &Vec<Rule> {
        &self.default_r_vec
    }
    pub fn get_fancy_r_vec(&self) -> &Vec<Rule> {
        &self.fancy_r_vec
    }
}
