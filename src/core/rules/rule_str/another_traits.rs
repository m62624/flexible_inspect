use super::*;
use crate::core::rules::traits::RuleBase;

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
            self.all_rules == other.all_rules
        }
    }

    impl Eq for SimpleRules {}
}

/// Here we implement a trait to retrieve the regular expression string directly
#[cfg(not(tarpaulin_include))]
mod as_ref_str_trait {
    use super::*;

    impl AsRef<str> for RegexRaw {
        fn as_ref(&self) -> &str {
            match self {
                RegexRaw::DefaultRegex(value) => value,
                RegexRaw::FancyRegex(value) => value,
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

/// Here we implement a trait to hash the data for `IndexSet`
#[cfg(not(tarpaulin_include))]
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
            if let Some(value) = &self.simple_rules {
                value.hash(state);
            }
        }
    }
}

#[cfg(any(feature = "serde", feature = "wasm"))]
mod serde_trait{
    use super::*;
    impl Serialize for RegexSetContainer {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Получаем вектор шаблонов
            let patterns = self.regex_set.patterns();
            // Сериализуем вектор шаблонов
            patterns.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for RegexSetContainer {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            // десериализуем вектор шаблонов
            let patterns: Vec<&str> =
                Deserialize::deserialize(deserializer)?;
            // компилируем в `RegexSet`
            let regex_set = regex::RegexSet::new(&patterns).unwrap();
            Ok(Self { regex_set })
        }
    }
}