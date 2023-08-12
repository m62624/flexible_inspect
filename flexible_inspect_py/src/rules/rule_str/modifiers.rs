use super::*;

#[pymethods]
impl PyRule {
    pub fn extend(&mut self, nested_rules: Vec<PyRule>) -> Self {
        self.0 = self
            .0
            .extend(nested_rules.into_iter().map(|rule| rule.into()));
        std::mem::take(self)
    }

    pub fn counter_is_equal(&mut self, count: usize) -> Self {
        self.0 = self.0.counter_is_equal(count);
        std::mem::take(self)
    }

    pub fn counter_more_than(&mut self, count: usize) -> Self {
        self.0 = self.0.counter_more_than(count);
        std::mem::take(self)
    }

    pub fn counter_less_than(&mut self, count: usize) -> Self {
        self.0 = self.0.counter_less_than(count);
        std::mem::take(self)
    }

    pub fn all_r_for_any_m(&mut self) -> Self {
        self.0 = self.0.all_r_for_any_m();
        std::mem::take(self)
    }

    pub fn any_r_for_all_m(&mut self) -> Self {
        self.0 = self.0.any_r_for_all_m();
        std::mem::take(self)
    }

    pub fn any_r_for_any_m(&mut self) -> Self {
        self.0 = self.0.any_r_for_any_m();
        std::mem::take(self)
    }
}
