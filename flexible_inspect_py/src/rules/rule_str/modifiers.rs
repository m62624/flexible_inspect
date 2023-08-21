use super::*;

#[pymethods]
impl PyRule {
    pub fn extend(&mut self, nested_rules: Vec<PyRule>) -> PyResult<Self> {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(
            m_self
                .0
                .map(|rule| rule.extend(nested_rules.into_iter().map(|rule| rule.into())))
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(m_self)
    }

    pub fn counter_is_equal(&mut self, count: usize) -> PyResult<Self> {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(
            m_self
                .0
                .map(|rule| rule.counter_is_equal(count))
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(m_self)
    }

    pub fn counter_more_than(&mut self, count: usize) -> PyResult<Self> {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(
            m_self
                .0
                .map(|rule| rule.counter_more_than(count))
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(m_self)
    }

    pub fn counter_less_than(&mut self, count: usize) -> PyResult<Self> {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(
            m_self
                .0
                .map(|rule| rule.counter_less_than(count))
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(m_self)
    }

    pub fn all_r_for_any_m(&mut self) -> PyResult<Self> {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(
            m_self
                .0
                .map(|rule| rule.all_r_for_any_m())
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(m_self)
    }

    pub fn any_r_for_all_m(&mut self) -> PyResult<Self> {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(
            m_self
                .0
                .map(|rule| rule.any_r_for_all_m())
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(m_self)
    }

    pub fn any_r_for_any_m(&mut self) -> PyResult<Self> {
        let mut m_self = std::mem::take(self);
        m_self.0 = Some(
            m_self
                .0
                .map(|rule| rule.any_r_for_any_m())
                .ok_or_else(|| PyErr::new::<exceptions::PyUnboundLocalError, _>(ERR_OPTION))?,
        );
        Ok(m_self)
    }
}
