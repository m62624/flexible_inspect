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
    /// Добавление дочерней вложенной строки
    pub fn extend(&mut self, key: &str, child: Rule) {
        self.rule_for_the_rule.insert(key.to_owned(), child);
    }
}
impl<'a> RuleIterator<'a> {
    pub fn new(root: &'a Rule) -> Self {
        let mut stack = Vec::new();
        stack.push(root);

        RuleIterator { stack }
    }
}

impl<'a> Iterator for RuleIterator<'a> {
    type Item = &'a Rule;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            for child in node.rule_for_the_rule.values() {
                self.stack.push(child);
            }
            Some(node)
        } else {
            None
        }
    }
}
