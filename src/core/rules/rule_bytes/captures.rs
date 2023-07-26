use log::info;

use crate::core::rules::{traits::RuleBase, CaptureData};
use crate::core::DEFAULT_CAPTURE;
use crate::RuleBytes;
use std::collections::{HashMap, HashSet};

pub fn find_captures<'a>(rule: &RuleBytes, capture: &'a [u8]) -> CaptureData<&'a [u8]> {
    let mut hashmap_for_error: HashMap<String, String> = HashMap::new();
    let mut text_for_capture: HashSet<&[u8]> = HashSet::new();
    let mut counter_value: usize = 0;
    // flag to check `Counter`
    let flag_check_counter = rule.content_unchecked().general_modifiers.counter.is_some();
    let re = regex::bytes::Regex::new(&rule.content_unchecked().str_bytes.as_ref()).unwrap();
    // get matches and increase `counter` as necessary
    re.captures_iter(capture).for_each(|capture| {
        if let Some(value) = capture.get(0) {
            hashmap_for_error
                .entry(DEFAULT_CAPTURE.into())
                .or_insert_with(|| bytes_to_byte_string(value.as_bytes()));
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
                        .or_insert_with(|| bytes_to_byte_string(value.as_bytes()));
                }
            }
        })
    });

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

fn bytes_to_byte_string(bytes: &[u8]) -> String {
    let byte_string: Vec<String> = bytes.iter().map(|byte| byte.to_string()).collect();
    format!("[{}]", byte_string.join(","))
}
