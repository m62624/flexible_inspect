use super::slice::RuleContext;
use super::*;

/// Реалзиуем методы для использование из `Python`
#[pymethods]
impl Rule {
    /// add subrules
    pub fn extend(&mut self, py: Python, nested_rules: PyObject) -> PyResult<Self> {
        // Проверяем, что это список
        if let Ok(list) = nested_rules.downcast::<types::PyList>(py) {
            // Создаем заранее два вектора, для простых и сложных правил
            let mut simple_collection: Vec<Rule> = Vec::new();
            let mut complex_collection: Vec<Rule> = Vec::new();
            // Заполняем эти векторы правилами
            RuleContext::slice_rules(
                RuleContext::Subelement(self),
                list,
                &mut simple_collection,
                &mut complex_collection,
            )?;
            // Если хотя бы один из векторов не пустой, то добавляем их в подправила
            if !simple_collection.is_empty() || !complex_collection.is_empty() {
                self.content_mut_unchecked().subrules = Some(Subrules::new(
                    SimpleRules::new(simple_collection),
                    complex_collection,
                ));
                // Возвращаем правило, которое было взято из владения
                return Ok(std::mem::take(self));
            }
        }
        Err(PyErr::new::<exceptions::PyTypeError, _>(format!(
            "`{}` -- expected `List` --> List [Rule, Rule, Rule]",
            nested_rules
        )))
    }

    /// All subrules should work successfully for all matches (text)
    pub fn mode_all_rules_for_all_matches(&mut self) -> Self {
        self.content_mut_unchecked().mod_match = ModeMatch::AllRulesForAllMatches;
        std::mem::take(self)
    }

    /// All subrules should work successfully for at least one match (text)
    pub fn mode_all_rules_for_at_least_one_match(&mut self) -> Self {
        self.content_mut_unchecked().mod_match = ModeMatch::AllRulesForAtLeastOneMatch;
        std::mem::take(self)
    }

    /// At least one rule should work successfully for all matches
    pub fn mode_one_rule_for_all_matches(&mut self) -> Self {
        self.content_mut_unchecked().mod_match = ModeMatch::OneRuleForAllMatches;
        std::mem::take(self)
    }

    /// At least one rule should work successfully for at least one match
    pub fn mode_at_least_one_rule_for_at_least_one_match(&mut self) -> Self {
        self.content_mut_unchecked().mod_match = ModeMatch::AtLeastOneRuleForAtLeastOneMatch;
        std::mem::take(self)
    }

    /// adding a match counter, exactly as many times as X is specified
    pub fn counter_is_equal(&mut self, count: usize) -> Self {
        self.content_mut_unchecked().counter = Some(Counter::Only(count));
        std::mem::take(self)
    }

    /// adding a counter of matches greater than or equal to X
    pub fn counter_more_than(&mut self, count: usize) -> Self {
        self.content_mut_unchecked().counter = Some(Counter::MoreThan(count));
        std::mem::take(self)
    }

    /// adding a counter of matches, less than or equal to X
    pub fn counter_less_than(&mut self, count: usize) -> Self {
        self.content_mut_unchecked().counter = Some(Counter::LessThan(count));
        std::mem::take(self)
    }
}

impl Rule {}
