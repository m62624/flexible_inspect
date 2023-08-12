use crate::error::ValidationError;

#[test]
fn error_iterator_t_0() {
    let error = vec![
        ValidationError::new(1, "error 1".into()),
        ValidationError::new(2, "error 2".into()),
        ValidationError::new(3, "error 3".into()),
    ];

    assert_eq!(error.into_iter().count(), 3);
}

#[test]
fn get_message_t_0() {
    let error = ValidationError::new(1, "error 1".into());
    assert_eq!(error.get_message(), "error 1");
}

#[test]
fn get_code_t_0() {
    let error = ValidationError::new(1, "error 1".into());
    assert_eq!(error.get_code(), 1);
}
