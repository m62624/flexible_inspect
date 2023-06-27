use super::*;

impl Rule {
    pub fn get_match_requirement(&self) -> PyResult<&MatchRequirement> {
        if let Some(value) = &self.requirement {
            return Ok(value);
        } else {
            return Err(take_self_error::absence_error());
        }
    }
    pub fn get_str_raw(&self) -> PyResult<&RegexRaw> {
        if let Some(value) = &self.str_raw {
            return Ok(value);
        } else {
            return Err(take_self_error::absence_error());
        }
    }
    pub fn get_op_subrules(&self) -> &Option<Subrules> {
        &self.subrules
    }
}
