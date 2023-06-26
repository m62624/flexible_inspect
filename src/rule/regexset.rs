use super::*;

impl Rule {
    pub fn get_regex_set(&self, text: &str) -> Option<Vec<&Rule>> {
        let mut temp = Vec::new();
        if let Some(subrules) = self.get_op_subrules() {
            return Some(
                regex::RegexSet::new(
                    subrules
                        .iter()
                        .filter_map(|rule| {
                            if let Some(value) = rule.get_op_str_raw() {
                                temp.push(rule);
                                return Some(value.get_str());
                            }
                            None
                        })
                        .collect::<Vec<&Box<str>>>(),
                )
                .unwrap()
                .matches(text)
                .iter()
                .map(move |id| temp[id])
                .collect::<Vec<_>>(),
            );
        }
        None
    }
}
