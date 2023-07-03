use super::slice::RuleContext;
use super::*;

#[pymethods]
impl Rule {
    /// Добавление подпправил
    pub fn extend(&mut self, py: Python, nested_rules: PyObject) -> PyResult<Self> {
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
                self.content_mut_unchecked().subrules = Some(Subrules::new(
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

    /// добавление счетчика совпадений, где условие : должно быть ровно `count` совпадений
    pub fn counter_is_equal(&mut self, count: usize) -> Self {
        self.content_mut_unchecked().counter = Some(Counter::Only(count));
        std::mem::take(self)
    }

    /// добавление счетчика совпадений, где условие : должно быть больше  (включительно)`count` совпадений
    pub fn counter_more_than(&mut self, count: usize) -> Self {
        self.content_mut_unchecked().counter = Some(Counter::MoreThan(count));
        std::mem::take(self)
    }

    /// добавление счетчика совпадений, где условие : должно быть меньше (включительно)`count` совпадений
    pub fn counter_less_than(&mut self, count: usize) -> Self {
        self.content_mut_unchecked().counter = Some(Counter::LessThan(count));
        std::mem::take(self)
    }
}
