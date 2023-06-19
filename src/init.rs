use super::*;
#[pymethods]
impl Rule {
    #[new]
    /// Создание корня дерева
    pub fn new(inner: String, requirements: MatchRequirement) -> Self {
        Rule {
            inner,
            requirements,
            rule_for_the_rule: HashMap::new(),
        }
    }
}
