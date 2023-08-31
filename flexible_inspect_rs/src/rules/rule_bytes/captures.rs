use crate::prelude::RuleBytes;
use crate::rules::{traits::RuleBase, CaptureData};
use crate::rules::{TypeStorageFormat, DEFAULT_CAPTURE};
use colored::Colorize;
use indexmap::IndexSet;
use log::info;
use std::collections::HashMap;
use std::marker::PhantomData;

pub fn find_captures<'a>(rule: &RuleBytes, capture: &'a [u8]) -> CaptureData<'a, &'a [u8]> {
    let mut hashmap_for_error: HashMap<String, String> = HashMap::new();
    let mut text_for_capture: IndexSet<&[u8]> = IndexSet::new();
    let mut text_for_capture_duplicate: Vec<&[u8]> = Vec::new();
    let mut counter_value: usize = 0;
    // flag to check `Counter`
    let flag_check_counter = rule.0.general_modifiers.counter.is_some();
    let re = regex::bytes::Regex::new(rule.0.str_bytes.as_ref()).unwrap();
    // get matches and increase `counter` as necessary
    re.captures_iter(capture).for_each(|capture| {
        if let Some(value) = capture.get(0) {
            hashmap_for_error
                .entry(DEFAULT_CAPTURE.into())
                .or_insert_with(|| format!("{:?}", value.as_bytes()));
            if rule.get_save_duplicates() {
                text_for_capture_duplicate.push(value.as_bytes());
            } else {
                text_for_capture.insert(value.as_bytes());
            }
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
        match (
            text_for_capture.is_empty(),
            text_for_capture_duplicate.is_empty(),
        ) {
            (true, true) | (false, false) => {
                info!(
                    "(capture) the rule `({}, {})` didn't find a match",
                    rule.get_str().yellow(),
                    format!("{:#?}", rule.get_requirement()).yellow()
                );
            }
            (true, false) => {
                info!(
                    "(capture) the rule `({}, {})` found a match: \n{:?}",
                    rule.get_str().yellow(),
                    format!("{:#?}", rule.get_requirement()).yellow(),
                    text_for_capture_duplicate
                );
            }
            (false, true) => {
                info!(
                    "(capture) the rule `({}, {})` found a match: \n{:?}",
                    rule.get_str().yellow(),
                    format!("{:#?}", rule.get_requirement()).yellow(),
                    text_for_capture
                );
            }
        }
    }
    CaptureData {
        text_for_capture: if rule.get_save_duplicates() {
            TypeStorageFormat::Multiple((text_for_capture_duplicate, PhantomData))
        } else {
            TypeStorageFormat::Single((text_for_capture, PhantomData))
        },
        hashmap_for_error,
        counter_value,
    }
}
