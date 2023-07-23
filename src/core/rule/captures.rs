use crate::DEFAULT_CAPTURE;

use super::{RegexRaw, Rule};
use std::collections::{HashMap, HashSet};

/// Structure for storing matches
#[derive(Debug)]
pub struct CaptureData<'s> {
    /// Stores name matches to populate data in error messages
    pub hashmap_for_error: HashMap<String, String>,
    /// Stores text matches for subrules
    pub text_for_capture: HashSet<&'s str>,
    /// Stores the number of matches, to check `Counter`.
    pub counter_value: usize,
}

impl<'s> CaptureData<'s> {
    pub fn find_captures(rule: &Rule, text: &'s str) -> Self {
        let mut hashmap_for_error = HashMap::new();
        let mut text_for_capture = HashSet::new();
        let mut counter_value: usize = 0;
        let flag_check_counter = rule.content_unchecked().mutable_modifiers.counter.is_some();
        // At first glance we see code duplication, but each `match` works with different structures
        match &rule.content_unchecked().str_with_type {
            RegexRaw::DefaultRegex(pattern) => {
                let re = regex::Regex::new(pattern).unwrap();
                // get matches and increase `counter` as necessary
                re.captures_iter(text).for_each(|capture| {
                    if let Some(value) = capture.get(0) {
                        hashmap_for_error
                            .entry(DEFAULT_CAPTURE.into())
                            .or_insert_with(|| value.as_str().into());
                        text_for_capture.insert(value.as_str());
                        // there can be several groups in one `regex`, but all of them
                        // they are needed to get the main match, so
                        // the counter is incremented only in `DEFAULT_CAPTURE`.
                        if flag_check_counter {
                            counter_value += 1;
                        }
                    }
                    // get matches by group names, to fill with data in error messages
                    re.capture_names().for_each(|name| {
                        if let Some(name) = name {
                            if let Some(value) = capture.name(name) {
                                hashmap_for_error
                                    .entry(name.into())
                                    .or_insert_with(|| value.as_str().into());
                            }
                        }
                    })
                });
            }
            RegexRaw::FancyRegex(pattern) => {
                let re = fancy_regex::Regex::new(pattern).unwrap();
                // get matches and increase `counter` as necessary
                re.captures_iter(text).for_each(|capture| {
                    if let Ok(capture) = capture {
                        if let Some(value) = capture.get(0) {
                            hashmap_for_error
                                .entry(DEFAULT_CAPTURE.into())
                                .or_insert_with(|| value.as_str().into());
                            text_for_capture.insert(value.as_str());
                            // there can be several groups in one `regex`, but all of them
                            // they are needed to get the main match, so
                            // the counter is incremented only in `DEFAULT_CAPTURE`.
                            if flag_check_counter {
                                counter_value += 1;
                            }
                        }
                        // get matches by group names, to fill with data in error messages
                        re.capture_names().for_each(|name| {
                            if let Some(name) = name {
                                if let Some(value) = capture.name(name) {
                                    hashmap_for_error
                                        .entry(name.into())
                                        .or_insert_with(|| value.as_str().into());
                                }
                            }
                        })
                    }
                });
            }
            RegexRaw::RegexBytes(pattern) => {}
        }
        Self {
            hashmap_for_error,
            text_for_capture,
            counter_value,
        }
    }
}
