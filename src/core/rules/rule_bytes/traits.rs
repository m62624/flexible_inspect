use super::*;

#[cfg(not(tarpaulin_include))]
mod partial_eq_eq_trait {
    use super::*;

    /*
    we implement only for simple rules, since we always create a `RegexSet` based on it with all rule regulars,
    which means they are identical anyway, so we only need to compare `all_rules`
     */
    impl PartialEq for SimpleRulesBytes {
        fn eq(&self, other: &Self) -> bool {
            self.all_rules == other.all_rules
        }
    }

    impl Eq for SimpleRulesBytes {}
}

#[cfg(not(tarpaulin_include))]
mod as_ref_str_trait {

    use super::*;

    impl AsRef<str> for RuleBytes {
        fn as_ref(&self) -> &str {
            self.content_unchecked().str_bytes.as_ref()
        }
    }

    impl AsRef<TakeRuleBytesForExtend> for &TakeRuleBytesForExtend {
        fn as_ref(&self) -> &TakeRuleBytesForExtend {
            self
        }
    }
}

#[cfg(not(tarpaulin_include))]
mod hash_trait {
    use super::*;

    /*
    we implement only for simple rules, since we always create a `RegexSet` based on it with all the regulars of the rule,
    which means that they are identical anyway, so we only need to hash `all_rules`
     */
    impl Hash for SimpleRulesBytes {
        fn hash<H: std::hash::Hasher>(&self, _: &mut H) {
            self.all_rules.hasher();
        }
    }
}
