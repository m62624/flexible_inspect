use super::{slice::RuleContext, *};

#[pymethods]
impl Rule {
    pub fn extend(&mut self, py: Python<'_>, nested_rules: PyObject) -> PyResult<Self> {
        // Проверяем, что это список
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            let (mut default_r_vec, mut fancy_r_vec) = (Vec::new(), Vec::new());
            RuleContext::slice_rules(
                RuleContext::Subelement(self),
                list,
                &mut default_r_vec,
                &mut fancy_r_vec,
            )?;
            if !default_r_vec.is_empty() || !fancy_r_vec.is_empty() {
                self.subrules = Some(Subrules::new(default_r_vec, fancy_r_vec));
            }
            return Ok(std::mem::take(self));
        }
        Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "`{}` -- expected `List` --> List [Rule, Rule, Rule]",
            nested_rules
        )))
    }
}
