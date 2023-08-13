use indexmap::IndexSet;
use std::collections::HashMap;

use crate::{
    prelude::*,
    rules::{next::NextStep, CaptureData},
};

/// true, true - MustBeFound
#[test]
fn test_next_runner_t_0() {
    let rule = Rule::new(r".+", MatchRequirement::MustBeFound)
        .extend([Rule::new(r"x", MatchRequirement::MustBeFound)]);
    let mut capture: CaptureData<String> = CaptureData {
        text_for_capture: IndexSet::from([String::from("x")]),
        hashmap_for_error: Default::default(),
        counter_value: Default::default(),
    };
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut capture),
        NextStep::Go
    );
}

/// true, false - MustBeFound
#[test]
fn test_next_runner_t_1() {
    let rule = Rule::new(r".+", MatchRequirement::MustBeFound);
    let mut capture: CaptureData<String> = CaptureData {
        text_for_capture: IndexSet::from([String::from("x")]),
        hashmap_for_error: Default::default(),
        counter_value: Default::default(),
    };
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut capture),
        NextStep::Finish
    );
}

/// false, true - MustBeFound
#[test]
fn test_next_runner_t_2() {
    let rule = Rule::new(r".+", MatchRequirement::MustBeFound)
        .extend([Rule::new(r"x", MatchRequirement::MustBeFound)]);
    let mut capture: CaptureData<String> = CaptureData {
        text_for_capture: IndexSet::new(),
        hashmap_for_error: Default::default(),
        counter_value: Default::default(),
    };
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut capture),
        NextStep::Error(None)
    );
}

/// false, false - MustBeFound
#[test]
fn test_next_runner_t_3() {
    let rule = Rule::new(r".+", MatchRequirement::MustBeFound);
    let mut capture: CaptureData<String> = CaptureData {
        text_for_capture: IndexSet::new(),
        hashmap_for_error: Default::default(),
        counter_value: Default::default(),
    };
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut capture),
        NextStep::Error(None)
    );
}

/// true, true - MustNotBeFound
#[test]
fn test_next_runner_t_4() {
    let rule = Rule::new(r".+", MatchRequirement::MustNotBeFound)
        .extend([Rule::new(r"x", MatchRequirement::MustNotBeFound)]);
    let mut capture: CaptureData<String> = CaptureData {
        text_for_capture: IndexSet::from([String::from("x")]),
        hashmap_for_error: Default::default(),
        counter_value: Default::default(),
    };
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut capture),
        NextStep::Go
    );
}

/// true, false - MustNotBeFound
#[test]
fn test_next_runner_t_5() {
    let rule = Rule::new(r".+", MatchRequirement::MustNotBeFound);
    let mut capture: CaptureData<String> = CaptureData {
        text_for_capture: IndexSet::from([String::from("x")]),
        hashmap_for_error: HashMap::from([(String::from("x"), String::from("Y"))]),
        counter_value: Default::default(),
    };
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut capture),
        NextStep::Error(Some(HashMap::from([(
            String::from("x"),
            String::from("Y")
        )])))
    );
}

/// false, true - MustNotBeFound
#[test]
fn test_next_runner_t_6() {
    let rule = Rule::new(r".+", MatchRequirement::MustNotBeFound)
        .extend([Rule::new(r"x", MatchRequirement::MustNotBeFound)]);
    let mut capture: CaptureData<String> = CaptureData {
        text_for_capture: IndexSet::new(),
        hashmap_for_error: HashMap::from([(String::from("x"), String::from("Y"))]),
        counter_value: Default::default(),
    };
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut capture),
        NextStep::Finish
    );
}

/// false, false - MustNotBeFound
#[test]
fn test_next_runner_t_7() {
    let rule = Rule::new(r".+", MatchRequirement::MustNotBeFound);
    let mut capture: CaptureData<String> = CaptureData {
        text_for_capture: IndexSet::new(),
        hashmap_for_error: HashMap::from([(String::from("x"), String::from("Y"))]),
        counter_value: Default::default(),
    };
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut capture),
        NextStep::Finish
    );
}
