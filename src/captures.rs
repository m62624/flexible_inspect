use super::rule::{RegexRaw, Rule};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct CaptureData<'s> {
    pub hashmap_for_error: HashMap<String, String>,
    pub text_for_capture: HashSet<&'s str>,
    pub counter_value: usize,
}

impl<'s> CaptureData<'s> {
    pub fn find_captures(rule: &Rule, text: &'s str) -> Self {
        let mut hashmap_for_error = HashMap::new();
        let mut text_for_capture = HashSet::new();
        let mut counter: usize = 0;
        let flag_check_counter = rule.content_unchecked().counter.is_some();
        match &rule.content_unchecked().str_with_type {
            RegexRaw::DefaultR(pattern) => {
                let re = regex::Regex::new(pattern).unwrap();
                re.captures_iter(text).for_each(|capture| {
                    if let Some(value) = capture.get(0) {
                        hashmap_for_error
                            .entry("main_capture".into())
                            .or_insert_with(|| value.as_str().into());
                        text_for_capture.insert(value.as_str());
                        if flag_check_counter {
                            counter += 1;
                        }
                    }
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
            RegexRaw::FancyR(pattern) => {
                let re = fancy_regex::Regex::new(pattern).unwrap();
                re.captures_iter(text).for_each(|capture| {
                    if let Ok(capture) = capture {
                        if let Some(value) = capture.get(0) {
                            hashmap_for_error
                                .entry("main_capture".into())
                                .or_insert_with(|| value.as_str().into());
                            text_for_capture.insert(value.as_str());
                            if flag_check_counter {
                                counter += 1;
                            }
                        }
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

        Self {
            hashmap_for_error,
            text_for_capture,
            counter_value: counter,
        }
    }

    pub fn is_some(&self) -> bool {
        !self.text_for_capture.is_empty()
    }
}
