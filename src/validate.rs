use super::*;
impl TemplateValidator {
    pub async fn core_validate(&self, text: String) -> PyResult<()> {
        let matches: Vec<_> = self
            .selected_simple_rules
            .matches(&text)
            .into_iter()
            .map(|match_idx| {
                self.all_simple_rules
                    .iter()
                    .find(|(x, _)| x.rule == self.selected_simple_rules.patterns()[match_idx])
                    .unwrap()
                    .0
            })
            .collect();
        //=====================
        dbg!(matches);
        //=====================

        Ok(())
    }
}
