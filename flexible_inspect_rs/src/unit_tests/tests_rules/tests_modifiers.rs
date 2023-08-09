use crate::{
    prelude::*,
    rules::{traits::RuleBase, Counter},
};

#[test]
pub fn test_modifiers_t_0() {
    let rule = Rule::new(r"\w+", MatchRequirement::MustBeFound).counter_is_equal(5);
    assert_eq!(rule.get_counter(), Some(Counter::Only(5)));
}

#[test]
pub fn test_modifiers_t_1() {
    let rule = Rule::new(r"\w+", MatchRequirement::MustBeFound).counter_more_than(5);
    assert_eq!(rule.get_counter(), Some(Counter::MoreThan(5)));
}

#[test]
pub fn test_modifiers_t_2() {
    let rule = Rule::new(r"\w+", MatchRequirement::MustBeFound).counter_less_than(5);
    assert_eq!(rule.get_counter(), Some(Counter::LessThan(5)));
}

#[test]
pub fn test_modifiers_t_3() {
    let rule = RuleBytes::new(r"\w+", MatchRequirement::MustBeFound).counter_is_equal(5);
    assert_eq!(rule.get_counter(), Some(Counter::Only(5)));
}

#[test]
pub fn test_modifiers_t_4() {
    let rule = RuleBytes::new(r"\w+", MatchRequirement::MustBeFound).counter_more_than(5);
    assert_eq!(rule.get_counter(), Some(Counter::MoreThan(5)));
}

#[test]
pub fn test_modifiers_t_5() {
    let rule = RuleBytes::new(r"\w+", MatchRequirement::MustBeFound).counter_less_than(5);
    assert_eq!(rule.get_counter(), Some(Counter::LessThan(5)));
}
