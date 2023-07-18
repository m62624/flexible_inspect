use super::*;
/// Реалзиуем методы для использование из `Python`
#[pymethods]
impl Rule {
    pub fn duplicate_matches(&mut self) -> Self {
        self.content_mut_unchecked().duplicate_matches = true;

        // ================= (LOG) =================
        debug!(
            "used the `duplicate_matches` modifier for `Rule` ( `{}` )",
            self.content_unchecked().str_with_type.as_ref()
        );
        // =========================================

        std::mem::take(self)
    }
}
