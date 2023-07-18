use super::*;

/// Реализация трейта по сравнению элементов
mod partial_eq_eq {
    use super::*;
    use crate::captures::{CaptureData, CaptureType};

    impl PartialEq for Subrules {
        fn eq(&self, other: &Self) -> bool {
            self.simple_rules == other.simple_rules && self.complex_rules == other.complex_rules
        }
    }

    impl PartialEq for SimpleRules {
        fn eq(&self, other: &Self) -> bool {
            self.all_rules == other.all_rules
        }
    }

    impl PartialEq for Counter {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Only(l0), Self::Only(r0)) => l0 == r0,
                (Self::MoreThan(l0), Self::MoreThan(r0)) => l0 == r0,
                (Self::LessThan(l0), Self::LessThan(r0)) => l0 == r0,
                _ => false,
            }
        }
    }

    impl PartialEq for ModeMatch {
        fn eq(&self, other: &Self) -> bool {
            core::mem::discriminant(self) == core::mem::discriminant(other)
        }
    }

    impl<'a> PartialEq for CaptureData<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.text_for_capture == other.text_for_capture
                && self.hashmap_for_error == other.hashmap_for_error
                && self.counter_value == other.counter_value
        }
    }

    impl<'a> PartialEq for CaptureType<'a> {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Single(l0), Self::Single(r0)) => l0 == r0,
                (Self::Multiple(l0), Self::Multiple(r0)) => l0 == r0,
                _ => false,
            }
        }
    }

    impl Eq for Subrules {}
    impl Eq for SimpleRules {}
    impl Eq for Counter {}
    impl Eq for ModeMatch {}
}

#[cfg(not(tarpaulin_include))]
/// Реализация трейта для получения ссылки
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
            self.content_unchecked().str_with_type.as_ref()
        }
    }

    impl AsRef<TakeRuleForExtend> for &TakeRuleForExtend {
        fn as_ref(&self) -> &TakeRuleForExtend {
            self
        }
    }
}
