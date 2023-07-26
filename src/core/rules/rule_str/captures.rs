use log::info;

use super::RegexRaw;
use crate::core::rules::{traits::RuleBase, CaptureData};
use crate::core::DEFAULT_CAPTURE;
use crate::Rule;
use std::collections::{HashMap, HashSet};

pub fn find_captures<'a>(rule: &Rule, capture: &'a str) -> CaptureData<&'a str> {
    let mut hashmap_for_error = HashMap::new();
    let mut text_for_capture: HashSet<&str> = HashSet::new();
    let mut counter_value: usize = 0;
    // flag to check `Counter`
    let flag_check_counter = rule.content_unchecked().general_modifiers.counter.is_some();
    // At first glance we see code duplication, but each `match` works with different structures
    match &rule.content_unchecked().str_with_type {
        RegexRaw::DefaultRegex(pattern) => {
            let re = regex::Regex::new(pattern).unwrap();
            // get matches and increase `counter` as necessary
            re.captures_iter(capture).for_each(|capture| {
                if let Some(value) = capture.get(0) {
                    hashmap_for_error
                        .entry(DEFAULT_CAPTURE.into())
                        .or_insert_with(|| value.as_str().into());
                    text_for_capture.insert(value.as_str());
                    // there can be several groups in one `regex`, but all of them
                    // they are needed to get the main match, so
                    // the increment is only in `main capture`.
                    if flag_check_counter {
                        counter_value += 1;
                    }
                }
                // get matches by group names to fill in the error message
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
            re.captures_iter(capture).for_each(|capture| {
                if let Ok(capture) = capture {
                    if let Some(value) = capture.get(0) {
                        hashmap_for_error
                            .entry(DEFAULT_CAPTURE.into())
                            .or_insert_with(|| value.as_str().into());
                        text_for_capture.insert(value.as_str());
                        // there can be several groups in one `regex`, but all of them
                        // they are needed to get the main match, so
                        // the increment is only in `main capture`.
                        if flag_check_counter {
                            counter_value += 1;
                        }
                    }
                    // get matches by group names to fill in the error message
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
    }

    // ============================= LOG =============================
    info!(
        "the {} rule found a match: \n{:#?}",
        rule.get_str(),
        text_for_capture
    );
    // ===============================================================

    CaptureData {
        text_for_capture,
        hashmap_for_error,
        counter_value,
    }
}
