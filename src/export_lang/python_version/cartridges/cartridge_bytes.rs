use super::*;

#[pyclass(name = "CartridgeBytes")]
#[derive(Default, Clone, Debug)]
pub struct PyCartridgeBytes(Cartridge<RuleBytes>);

impl From<PyCartridgeBytes> for Cartridge<RuleBytes> {
    fn from(value: PyCartridgeBytes) -> Self {
        value.0
    }
}

#[pymethods]
impl PyCartridgeBytes {
    #[new]
    pub fn new(py: Python, id: i64, message: String, root_rules: PyObject) -> PyResult<Self> {
        Ok(Self(Cartridge::new(
            id,
            message,
            PyRuleBytes::_to_rust_for_extend::<PyRuleBytes>(py, root_rules, "RuleBytes")?,
        )))
    }

    /// modifier to change the rule matching mode,
    /// `all rules` must pass the test for at least `one match`
    pub fn mode_all_rules_for_at_least_one_match(&mut self) -> Self {
        self.0 = self.0.mode_all_rules_for_at_least_one_match();
        std::mem::take(self)
    }

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for `all matches`
    pub fn mode_at_least_one_rule_for_all_matches(&mut self) -> Self {
        self.0 = self.0.mode_at_least_one_rule_for_all_matches();
        std::mem::take(self)
    }

    /// modifier to change the rule matching mode,
    /// at least `one rule` must pass the test for at least `one match`
    pub fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self {
        self.0 = self.0.mode_at_least_one_rule_for_at_least_one_match();
        std::mem::take(self)
    }

    /// Get the `error code`
    pub fn get_id(&self) -> i64 {
        self.0.id
    }

    /// Get an `error message` with data
    pub fn get_message(&self) -> &str {
        &self.0.message
    }
}
