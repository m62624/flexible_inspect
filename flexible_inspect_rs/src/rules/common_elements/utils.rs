use super::*;

mod impl_methods {
    use super::*;

    mod special_type_capture {
        use super::*;

        impl<'a> IntoSpecificCaptureType<'a> for &'a str {
            fn as_str(&self) -> Option<&'a str> {
                Some(self)
            }

            fn as_bytes(&self) -> Option<&'a [u8]> {
                None
            }
        }

        impl<'a> IntoSpecificCaptureType<'a> for &'a [u8] {
            fn as_str(&self) -> Option<&'a str> {
                None
            }

            fn as_bytes(&self) -> Option<&'a [u8]> {
                Some(self)
            }
        }
    }

    impl<R: RuleBase> SlisedRules<R> {
        /// A method for checking if there are any rules
        pub fn is_some(&self) -> bool {
            !self.smr_must_be_found.is_empty()
                || !self.smr_must_not_be_found_with_subrules.is_empty()
                || !self.smr_must_not_be_found_without_subrules.is_empty()
                || !self.cmr.is_empty()
        }
    }

    impl<'a, T: IntoSpecificCaptureType<'a>> TypeStorageFormat<'a, T> {
        pub fn len(&self) -> usize {
            match self {
                Self::Single((set, _)) => set.len(),
                Self::Multiple((vec, _)) => vec.len(),
            }
        }

        pub fn iter(&self) -> Box<dyn Iterator<Item = T> + '_> {
            match self {
                Self::Single((set, _)) => Box::new(set.iter().copied()),
                Self::Multiple((vec, _)) => Box::new(vec.iter().copied()),
            }
        }
    }
}

mod impl_traits {
    use super::*;

    impl AsRef<str> for RegexRaw {
        fn as_ref(&self) -> &str {
            match self {
                RegexRaw::DefaultRegex(value) => value,
                RegexRaw::FancyRegex(value) => value,
                RegexRaw::BytesRegex(value) => value,
            }
        }
    }
}
