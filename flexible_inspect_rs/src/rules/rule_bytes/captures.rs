use crate::prelude::RuleBytes;
use crate::rules::DEFAULT_CAPTURE;
use crate::rules::{traits::RuleBase, CaptureData};
use colored::Colorize;
use indexmap::IndexSet;
use log::info;
use std::collections::HashMap;

pub fn find_captures<'a>(rule: &RuleBytes, capture: &'a [u8]) -> CaptureData<&'a [u8]> {
    let mut hashmap_for_error: HashMap<String, String> = HashMap::new();
    let mut text_for_capture: IndexSet<&[u8]> = IndexSet::new();
    let mut counter_value: usize = 0;
    // flag to check `Counter`
    let flag_check_counter = rule.content_unchecked().general_modifiers.counter.is_some();
    let re = regex::bytes::Regex::new(rule.content_unchecked().str_bytes.as_ref()).unwrap();
    // get matches and increase `counter` as necessary
    re.captures_iter(capture).for_each(|capture| {
        if let Some(value) = capture.get(0) {
            hashmap_for_error
                .entry(DEFAULT_CAPTURE.into())
                .or_insert_with(|| format!("{:?}", value.as_bytes()));
            text_for_capture.insert(value.as_bytes());
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
                        .or_insert_with(|| format!("{:?}", value.as_bytes()));
                }
            }
        })
    });

    if log::log_enabled!(log::Level::Info) {
        if text_for_capture.is_empty() {
            info!(
                "the `({}, {})` rule didn't find a match",
                rule.get_str().yellow(),
                format!("{:#?}", rule.get_requirement()).yellow()
            );
        } else {
            info!(
                "the rule `({}, {})` found a match: \n{:#?}",
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
