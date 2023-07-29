use std::collections::HashMap;

use crate::core::message::filling_message;

#[test]
fn test_filling_message_t_0() {
    let message_template = "Hello, {x}!".to_owned();
    let error_data = Some(HashMap::from([("x".to_owned(), "world".to_owned())]));
    assert_eq!(
        filling_message(&message_template, error_data),
        "Hello, world!"
    );
}

#[test]
fn test_filling_message_t_1() {
    let message_template = "Hello, {x}!".to_owned();
    let error_data = Some(HashMap::from([("y".to_owned(), "world".to_owned())]));
    assert_eq!(filling_message(&message_template, error_data), "Hello, ___!");
}

#[test]
fn test_filling_message_t_2() {
    let message_template = "Hello, {x}!".to_owned();
    let error_data = None;
    assert_eq!(filling_message(&message_template, error_data), "Hello, ___!");
}

#[test]
fn test_filling_message_t_3() {
    let message_template = "{x} {y} {z}".to_owned();
    let error_data = Some(HashMap::from([
        ("x".to_owned(), "1".to_owned()),
        ("y".to_owned(), "2".to_owned()),
        ("z".to_owned(), "3".to_owned()),
    ]));
    assert_eq!(filling_message(&message_template, error_data), "1 2 3");
}
