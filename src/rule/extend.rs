use super::slice::RuleContext;
use super::*;

#[pymethods]
impl Rule {
    pub fn extend(&mut self, py: Python<'_>, nested_rules: PyObject) -> PyResult<Self> {
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            let mut simple_collection: Vec<Rule> = Vec::new();
            let mut complex_collection: Vec<Rule> = Vec::new();
            RuleContext::slice_rules(
                RuleContext::Subelement(self),
                list,
                &mut simple_collection,
                &mut complex_collection,
            )?;
            if !simple_collection.is_empty() || !complex_collection.is_empty() {
                self.get_content_mut().unwrap().subrules = Some(Subrules::new(
                    SimpleRules::new(simple_collection),
                    complex_collection,
                ));
                return Ok(std::mem::take(self));
            }
        }
        Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "`{}` -- expected `List` --> List [Rule, Rule, Rule]",
            nested_rules
        )))
    }
}
