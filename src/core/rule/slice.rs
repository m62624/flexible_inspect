use super::*;

/// the struct for sorting all nested rules
pub struct SlisedRules {
    pub simple_rules: IndexSet<Rule>,
    pub complex_rules: IndexSet<Rule>,
    pub bytes_rules: IndexSet<Rule>,
}

impl SlisedRules {
    /// the method for sorting all nested rules
    pub fn slice_rules(all_rules: IndexSet<Rule>) -> SlisedRules {
        /*
        We do not save directly via `insert` to `IndexSet` as we would lose the ordering,
        we need the order to properly check rules that are not in `RegexSet` when searching through the index.
        `RegexSet` is implemented in `simple_regex` and `bytes_regex`,
        so there is no need to follow the queue for `complex rules`
        */
        let mut o_simple_rules = Vec::new();
        let mut o_complex_rules = IndexSet::new();
        let mut o_bytes_rules = Vec::new();

        all_rules
            .into_iter()
            .for_each(|rule| match rule.content_unchecked().str_with_type {
                RegexRaw::DefaultRegex(_) => o_simple_rules.push(rule),
                RegexRaw::FancyRegex(_) => {
                    o_complex_rules.insert(rule);
                }
                RegexRaw::RegexBytes(_) => o_bytes_rules.push(rule),
            });

        let o_simple_rules: IndexSet<_> = o_simple_rules.into_iter().collect();
        let o_bytes_rules: IndexSet<_> = o_bytes_rules.into_iter().collect();

        SlisedRules {
            simple_rules: o_simple_rules,
            complex_rules: o_complex_rules,
            bytes_rules: o_bytes_rules,
        }
    }

    /// the method for checking if there are any rules
    pub fn is_some(&self) -> bool {
        if !self.simple_rules.is_empty()
            || !self.complex_rules.is_empty()
            || !self.bytes_rules.is_empty()
        {
            true
        } else {
            false
        }
    }
}
