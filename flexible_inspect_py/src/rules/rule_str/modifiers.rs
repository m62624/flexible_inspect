use super::*;

#[pymethods]
impl PyRule {
    pub fn extend(&mut self, nested_rules: Vec<PyRule>) -> Self {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(
            m_self
                .0
                .expect(ERR_OPTION)
                .extend(nested_rules.into_iter().map(|rule| rule.into())),
        );
        m_self
    }

    pub fn counter_is_equal(&mut self, count: usize) -> Self {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(m_self.0.expect(ERR_OPTION).counter_is_equal(count));
        std::mem::take(self)
    }

    pub fn counter_more_than(&mut self, count: usize) -> Self {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(m_self.0.expect(ERR_OPTION).counter_more_than(count));
        std::mem::take(self)
    }

    pub fn counter_less_than(&mut self, count: usize) -> Self {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(m_self.0.expect(ERR_OPTION).counter_less_than(count));
        std::mem::take(self)
    }

    pub fn all_r_for_any_m(&mut self) -> Self {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(m_self.0.expect(ERR_OPTION).all_r_for_any_m());
        std::mem::take(self)
    }

    pub fn any_r_for_all_m(&mut self) -> Self {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(m_self.0.expect(ERR_OPTION).any_r_for_all_m());
        std::mem::take(self)
    }

    pub fn any_r_for_any_m(&mut self) -> Self {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(m_self.0.expect(ERR_OPTION).any_r_for_any_m());
        std::mem::take(self)
    }
}
