use futures::StreamExt;

use crate::error::{iterator::ValidationErrorIterator, BaseValidationError};

#[test]
fn error_iterator_t_0() {
    let error = ValidationErrorIterator::new(vec![
        Box::new(BaseValidationError::new(1, "error 1".into())),
        Box::new(BaseValidationError::new(2, "error 2".into())),
        Box::new(BaseValidationError::new(3, "error 3".into())),
    ]);

    assert_eq!(error.into_iter().count(), 3);
}

#[test]
fn get_message_t_0() {
    let error = ValidationErrorIterator::new(vec![Box::new(BaseValidationError::new(
        1,
        "error 1".into(),
    ))]);
    assert_eq!(error.into_iter().next().unwrap().get_message(), "error 1");
}

#[test]
fn get_code_t_0() {
    let error = ValidationErrorIterator::new(vec![Box::new(BaseValidationError::new(
        1,
        "error 1".into(),
    ))]);
    assert_eq!(error.into_iter().next().unwrap().get_code(), 1);
}

#[test]
fn iterator_as_ref_t_0() {
    let iterator = ValidationErrorIterator::new(vec![
        Box::new(BaseValidationError::new(1, "error 1".into())),
        Box::new(BaseValidationError::new(2, "error 2".into())),
    ]);

    assert_eq!(iterator.as_ref().into_iter().count(), 2);
    assert_eq!(iterator.as_ref().into_iter().count(), 2);
}

#[test]
fn iterator_as_ref_t_1() {
    let iterator = ValidationErrorIterator::new(vec![
        Box::new(BaseValidationError::new(1, "error 1".into())),
        Box::new(BaseValidationError::new(2, "error 2".into())),
    ]);

    assert_eq!(iterator.as_ref().into_iter().next().unwrap().get_code(), 1);
    assert_eq!(iterator.as_ref().into_iter().next().unwrap().get_code(), 1);
}

#[test]
fn iterator_iter_t_0() {
    let mut iterator = ValidationErrorIterator::new(vec![
        Box::new(BaseValidationError::new(1, "error 1".into())),
        Box::new(BaseValidationError::new(2, "error 2".into())),
    ]);

    assert!(iterator.next().is_some());
    assert!(iterator.next().is_some());
    assert!(iterator.next().is_none());
}

#[async_std::test]
async fn iterator_async_iter_t_0() {
    let iterator = ValidationErrorIterator::new(vec![
        Box::new(BaseValidationError::new(1, "error 1".into())),
        Box::new(BaseValidationError::new(2, "error 2".into())),
    ]);
    let mut async_iter = iterator.async_into_iter().await;
    while let Some(item) = async_iter.next().await {
        assert!(item.get_code() == 1 || item.get_code() == 2);
    }
}
