use super::*;
/// Реалзиуем методы для использование из `Python`
#[pymethods]
impl Rule {
    /// adding a match counter, exactly as many times as X is specified
    pub fn counter_is_equal(&mut self, count: usize) -> Self {
        self.content_mut_unchecked().counter = Some(Counter::Only(count));

        // ================= (LOG) =================
        debug!(
            "used the `counter_is_equal` modifier for `Rule` ( `{}` )",
            self.content_unchecked().str_with_type.as_ref()
        );
        // =========================================

        std::mem::take(self)
    }

    /// adding a counter of matches greater than or equal to X
    pub fn counter_more_than(&mut self, count: usize) -> Self {
        self.content_mut_unchecked().counter = Some(Counter::MoreThan(count));

        // ================= (LOG) =================
        debug!(
            "used the `counter_more_than` modifier for `Rule` ( `{}` )",
            self.content_unchecked().str_with_type.as_ref()
        );
        // =========================================

        std::mem::take(self)
    }

    /// adding a counter of matches, less than or equal to X
    pub fn counter_less_than(&mut self, count: usize) -> Self {
        self.content_mut_unchecked().counter = Some(Counter::LessThan(count));

        // ================= (LOG) =================
        debug!(
            "used the `counter_less_than` modifier for `Rule` ( `{}` )",
            self.content_unchecked().str_with_type.as_ref()
        );
        // =========================================

        std::mem::take(self)
    }
}
