use super::*;

/// Here we implement a traite for comparing elements to use in `collections` and using `contains`.
#[cfg(not(tarpaulin_include))]
mod partial_eq_eq_trait {
    use super::*;

    /*
    we implement only for simple rules, since we always create a `RegexSet` based on it with all rule regulars,
    which means they are identical anyway, so we only need to compare `all_rules`
     */
    impl PartialEq for SimpleRules {
        fn eq(&self, other: &Self) -> bool {
            self.smr_must_be_found == other.smr_must_be_found
                && self.smr_must_not_be_found_with_subrules
                    == other.smr_must_not_be_found_with_subrules
                && self.smr_must_not_be_found_without_subrules
                    == other.smr_must_not_be_found_without_subrules
        }
    }

    impl Eq for SimpleRules {}
}

/// Here we implement a trait to retrieve the regular expression string directly
#[cfg(not(tarpaulin_include))]
mod as_ref_str_trait {
    use super::*;

    impl AsRef<str> for Rule {
        fn as_ref(&self) -> &str {
            self.0.str_with_type.as_ref()
        }
    }

    impl AsRef<TakeRuleForExtend> for &TakeRuleForExtend {
        fn as_ref(&self) -> &TakeRuleForExtend {
            self
        }
    }
}

/// Here we implement a trait to hash the data for `IndexSet`
#[cfg(not(tarpaulin_include))]
mod hash_trait {
    use super::*;

    /*
    we implement only for simple rules, since we always create a `RegexSet` based on it with all the regulars of the rule,
    which means that they are identical anyway, so we only need to hash `all_rules`
     */
    impl Hash for SimpleRules {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.smr_must_be_found
                .iter()
                .chain(&self.smr_must_not_be_found_with_subrules)
                .chain(&self.smr_must_not_be_found_without_subrules)
                .for_each(|item| item.hash(state));
        }
    }

    impl Hash for Subrules {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.simple_rules.hash(state);
            match &self.complex_rules {
                Some(complex_rules_value) => {
                    complex_rules_value.iter().for_each(|rule| rule.hash(state));
                }
                None => 0.hash(state),
            }
        }
    }
}

/// Here we implement a trait for `RegexSet` serialization and deserialization
#[cfg(feature = "serde")]
mod serde_trait {
    use super::*;
    impl Serialize for RegexSetContainer {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Serialize the vector
            self.regex_set.patterns().serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for RegexSetContainer {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            // Get a vector of patterns
            let patterns: Vec<String> = Deserialize::deserialize(deserializer)?;
            // Serialize the vector
            let regex_set = regex::RegexSet::new(patterns).map_err(serde::de::Error::custom)?;
            Ok(Self { regex_set })
        }
    }
}
