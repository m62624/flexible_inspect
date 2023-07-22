use super::*;

///
mod partial_eq_eq_trait {
    use super::*;

    /*
    we implement only for simple rules, since we always create a `RegexSet` based on it with all rule regulars,
    which means they are identical anyway, so we only need to compare `all_rules`
     */
    impl PartialEq for SimpleRules {
        fn eq(&self, other: &Self) -> bool {
            self.all_rules == other.all_rules
        }
    }

    impl Eq for SimpleRules {}
}

#[cfg(not(tarpaulin_include))]
/// Implementation of the trait to get the `str pattern` from the `Rule` structure.
mod as_ref_str_trait {

    use super::*;

    impl AsRef<str> for RegexRaw {
        fn as_ref(&self) -> &str {
            match self {
                RegexRaw::DefaultRegex(value) => value,
                RegexRaw::FancyRegex(value) => value,
                RegexRaw::RegexBytes(value) => value,
            }
        }
    }

    impl AsRef<str> for Rule {
        fn as_ref(&self) -> &str {
            self.content_unchecked().str_with_type.as_ref()
        }
    }

    impl AsRef<TakeRuleForExtend> for &TakeRuleForExtend {
        fn as_ref(&self) -> &TakeRuleForExtend {
            self
        }
    }
}

mod hash_trait {
    use super::*;

    /*
    we implement only for simple rules, since we always create a `RegexSet` based on it with all the regulars of the rule,
    which means that they are identical anyway, so we only need to hash `all_rules`
     */
    impl Hash for SimpleRules {
        fn hash<H: std::hash::Hasher>(&self, _: &mut H) {
            self.all_rules.hasher();
        }
    }

    impl Hash for Subrules {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.simple_rules.hash(state);
            if let Some(value) = &self.complex_rules {
                value.hasher();
            }
        }
    }
}

// #[test]
// fn hahs_subrlues() {
//     let subrule_1 = Subrules {
//         simple_rules: IndexSet::from([1]),
//         complex_rules: todo!(),
//     };
// }
