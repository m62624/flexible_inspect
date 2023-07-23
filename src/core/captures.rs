use super::rule::{RegexRaw, Rule};
use super::DEFAULT_CAPTURE;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct StrOrBytes<'s> {
    pub text_for_capture: Option<HashSet<&'s str>>,
    pub bytes_for_capture: Option<HashSet<&'s [u8]>>,
}

impl<'s> StrOrBytes<'s> {
    fn new(text_for_capture: HashSet<&'s str>, bytes_for_capture: HashSet<&'s [u8]>) -> Self {
        Self {
            text_for_capture: match !text_for_capture.is_empty() {
                true => Some(text_for_capture),
                false => None,
            },
            bytes_for_capture: match !bytes_for_capture.is_empty() {
                true => Some(bytes_for_capture),
                false => None,
            },
        }
    }
}

/// Structure for storing matches
#[derive(Debug)]
pub struct CaptureData<'s> {
    /// Stores name matches to populate data in error messages
    pub hashmap_for_error: HashMap<String, String>,
    /// Stores text matches for subrules
    pub data_for_capture: StrOrBytes<'s>,
    /// Stores the number of matches, to check `Counter`.
    pub counter_value: usize,
}

impl<'s> CaptureData<'s> {
    fn find_captures(rule: &Rule, text_str: Option<&'s str>, text_bytes: Option<&[u8]>) -> Self {
        let mut hashmap_for_error = HashMap::new();
        // ==========================================================
        let mut text_for_capture: HashSet<&str> = HashSet::new();
        let mut bytes_for_capture: HashSet<&[u8]> = HashSet::new();
        // ==========================================================
        let flag_check_counter = rule.content_unchecked().mutable_modifiers.counter.is_some();
        let mut counter_value: usize = 0;
        // At first glance we see code duplication, but each `match` works with different structures
        match &rule.content_unchecked().str_with_type {
            RegexRaw::DefaultRegex(pattern) => {
                let re = regex::Regex::new(pattern).unwrap();
                // re.captures_iter()
            }
            RegexRaw::FancyRegex(pattern) => {}
            RegexRaw::RegexBytes(pattern) => {}
        }
        Self {
            hashmap_for_error,
            counter_value,
            data_for_capture: StrOrBytes::new(text_for_capture, bytes_for_capture),
        }
    }
}
