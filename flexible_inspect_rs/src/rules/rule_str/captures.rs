use super::RegexRaw;
use super::*;
use crate::prelude::Rule;
use crate::rules::DEFAULT_CAPTURE;
use crate::rules::{traits::RuleBase, CaptureData};
use indexmap::IndexSet;
use log::info;
use std::collections::HashMap;

pub fn find_captures<'a>(rule: &Rule, capture: &'a str) -> CaptureData<&'a str> {
    let mut hashmap_for_error = HashMap::new();
    let mut text_for_capture: IndexSet<&str> = IndexSet::new();
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

    if log::log_enabled!(log::Level::Info) {
        if text_for_capture.is_empty() {
            info!(
                "(capture) the rule `({}, {})` didn't find a match",
                rule.get_str().yellow(),
                format!("{:#?}", rule.get_requirement()).yellow()
            );
        } else {
            info!(
                "(capture) the rule `({}, {})` found a match: \n{:#?}",
                rule.get_str().yellow(),
                format!("{:#?}", rule.get_requirement()).yellow(),
                text_for_capture
            )
        }
    }

    CaptureData {
        text_for_capture,
        hashmap_for_error,
        counter_value,
    }
}
