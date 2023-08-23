use super::*;

#[pymethods]
impl PyRuleBytes {
    pub fn extend(&mut self, nested_rules: Vec<PyRuleBytes>) -> PyResult<Self> {
        let mut mem_self: PyRuleBytes = self.try_into()?;
        let nested_rules = nested_rules
            .into_iter()
            .map(|rule| rule.try_into())
            .collect::<PyResult<Vec<RuleBytes>>>()?;
        mem_self.0 = mem_self.0.map(|rule| rule.extend(nested_rules));
        Ok(mem_self)
    }

    pub fn counter_is_equal(&mut self, count: usize) -> PyResult<Self> {
        let mut mem_self: PyRuleBytes = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.counter_is_equal(count));
        Ok(mem_self)
    }

    pub fn counter_more_than(&mut self, count: usize) -> PyResult<Self> {
        let mut mem_self: PyRuleBytes = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.counter_more_than(count));
        Ok(mem_self)
    }

    pub fn counter_less_than(&mut self, count: usize) -> PyResult<Self> {
        let mut mem_self: PyRuleBytes = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.counter_less_than(count));
        Ok(mem_self)
    }

    pub fn all_r_for_any_m(&mut self) -> PyResult<Self> {
        let mut mem_self: PyRuleBytes = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.all_r_for_any_m());
        Ok(mem_self)
    }

    pub fn any_r_for_all_m(&mut self) -> PyResult<Self> {
        let mut mem_self: PyRuleBytes = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.any_r_for_all_m());
        Ok(mem_self)
    }

    pub fn any_r_for_any_m(&mut self) -> PyResult<Self> {
        let mut mem_self: PyRuleBytes = self.try_into()?;
        mem_self.0 = mem_self.0.map(|rule| rule.any_r_for_any_m());
        Ok(mem_self)
    }
}
