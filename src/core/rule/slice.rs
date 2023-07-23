use super::*;

impl SlisedRules {
    /// The method for sorting all nested rules
    pub fn slice_rules<T: IntoIterator<Item = Rule>>(
        root_rule: &Rule,
        all_rules: T,
    ) -> SlisedRules {
        /*
        We do not save directly via `insert` to `IndexSet` as we would lose the ordering,
        we need the order to properly check rules that are not in `RegexSet` when searching through the index.
        `RegexSet` is implemented in `simple_regex` and `bytes_regex`,
        so there is no need to follow the queue for `complex rules`
        */
        let mut o_simple_rules = Vec::new();
        let mut o_complex_rules = Vec::new();
        let mut o_bytes_rules = Vec::new();

        let mut rule_access = |root_type: &Rule, rule: &Rule| {
            // Common part of the panic message
            let panic_msg = format!(
"\n\n\nonly regular expressions for `&str` strings can be added to a regular expression for `&str` strings, not for &[u8] bytes.
=============================
Root rule \"( {:?},{:#?} )\"
=============================
Problem rule \"( {:?},{:#?} )\"
=============================
If you want to add a regular expression for &[u8] bytes, then use the `for_bytes` method.\n\n\n",
            root_type.content_unchecked().str_with_type,
            root_rule.content_unchecked().mutable_modifiers.requirement, 
            rule.content_unchecked().str_with_type,
            rule.content_unchecked().mutable_modifiers.requirement
        );
            match root_type.content_unchecked().str_with_type {
                RegexRaw::DefaultRegex(_) => {
                    if let RegexRaw::RegexBytes(_) = rule.content_unchecked().str_with_type {
                        panic!("{}", panic_msg)
                    }
                }
                RegexRaw::FancyRegex(_) => {
                    if let RegexRaw::RegexBytes(_) = rule.content_unchecked().str_with_type {
                        panic!("{}", panic_msg)
                    }
                }
                RegexRaw::RegexBytes(_) => {
                    if let RegexRaw::DefaultRegex(_) | RegexRaw::FancyRegex(_) =
                        rule.content_unchecked().str_with_type
                    {
                        panic!(
"\n\n\nonly regular expressions for &[u8] bytes can be added to a regular expression for &[u8] bytes, not for `&str` strings.
=============================
Root rule \"( {:?},{:#?} )\"
=============================
Problem rule \"( {:?},{:#?} )\"
=============================\n\n\n",
            root_type.content_unchecked().str_with_type,
            root_rule.content_unchecked().mutable_modifiers.requirement, 
            rule.content_unchecked().str_with_type,
            rule.content_unchecked().mutable_modifiers.requirement
)
                    }
                }
            }
        };

        all_rules
            .into_iter()
            .for_each(|rule| {
                rule_access(&root_rule, &rule);
                match rule.content_unchecked().str_with_type{
                    RegexRaw::DefaultRegex(_) => o_simple_rules.push(rule),
                    RegexRaw::FancyRegex(_) => o_complex_rules.push(rule),
                    RegexRaw::RegexBytes(_) => o_bytes_rules.push(rule),
                }
            });

        let o_simple_rules: IndexSet<_> = o_simple_rules.into_iter().collect();
        let o_bytes_rules: IndexSet<_> = o_bytes_rules.into_iter().collect();

        SlisedRules {
            simple_rules: o_simple_rules,
            complex_rules: o_complex_rules,
            bytes_rules: o_bytes_rules,
        }
    }

    /// A method for checking if there are any rules
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
