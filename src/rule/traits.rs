use super::*;

mod partial_eq_eq {
    use super::*;

    impl PartialEq for Subrules {
        fn eq(&self, other: &Self) -> bool {
            self.simple_rules == other.simple_rules && self.complex_rules == other.complex_rules
        }
    }

    impl Eq for Subrules {}

    impl PartialEq for SimpleRules {
        fn eq(&self, other: &Self) -> bool {
            self.all_rules == other.all_rules
        }
    }

    impl Eq for SimpleRules {}
}

mod as_ref_str {
    use super::*;

    impl AsRef<str> for RegexRaw {
        fn as_ref(&self) -> &str {
            match self {
                RegexRaw::DefaultR(value) => value,
                RegexRaw::FancyR(value) => value,
            }
        }
    }

    impl AsRef<str> for Rule {
        fn as_ref(&self) -> &str {
            self.get_content().unwrap().str_with_type.as_ref()
        }
    }
}
