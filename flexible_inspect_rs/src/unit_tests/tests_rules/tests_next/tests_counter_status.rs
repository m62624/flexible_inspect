use super::*;

/// Проверяем счетчик, где найдено ровно 4 (включительно) совпадения
#[test]
fn fn_counter_status_t_0() {
    let rule = Rule::new(r"\w", MatchRequirement::MustBeFound).counter_is_equal(4);
    let mut captures = Rule::find_captures(&rule, &"a b c d");
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut captures),
        NextStep::Finish
    );
}

/// Проверяем счетчик, где найдено должно быть 4 (включительно) совпадения, но всего найдено 3
#[test]
fn fn_counter_status_t_1() {
    let rule = Rule::new(r"\w", MatchRequirement::MustBeFound).counter_is_equal(4);
    let mut captures = Rule::find_captures(&rule, &"a b c");
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut captures),
        NextStep::Error(Some(HashMap::from([(DEFAULT_CAPTURE.into(), "a".into())])))
    );
}

/// Проверяем счетчик, где найдено должно быть меньше 4 (включительно) совпадения
#[test]
fn fn_counter_status_t_2() {
    let rule = Rule::new(r"\w", MatchRequirement::MustBeFound).counter_less_than(4);
    let mut captures = Rule::find_captures(&rule, &"a b c");
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut captures),
        NextStep::Finish
    );
}

/// Проверяем счетчик, где найдено должно быть меньше 4 (включительно) совпадения, но всего найдено 5
#[test]
fn fn_counter_status_t_3() {
    let rule = Rule::new(r"\w", MatchRequirement::MustBeFound).counter_less_than(4);
    let mut captures = Rule::find_captures(&rule, &"a b c d e");
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut captures),
        NextStep::Error(Some(HashMap::from([(DEFAULT_CAPTURE.into(), "a".into())])))
    );
}

/// Проверяем счетчик, где найдено должно быть больше 4 (включительно) совпадения
#[test]
fn fn_counter_status_t_4() {
    let rule = Rule::new(r"\w", MatchRequirement::MustBeFound).counter_more_than(4);
    let mut captures = Rule::find_captures(&rule, &"a b c d e f g");
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut captures),
        NextStep::Finish
    );
}

/// Проверяем счетчик, где найдено должно быть больше 4 (включительно) совпадения, но всего найдено 3
#[test]
fn fn_counter_status_t_5() {
    let rule = Rule::new(r"\w", MatchRequirement::MustBeFound).counter_more_than(4);
    let mut captures = Rule::find_captures(&rule, &"a b c");
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut captures),
        NextStep::Error(Some(HashMap::from([(DEFAULT_CAPTURE.into(), "a".into())])))
    );
}

/// true,true, MustBeFound, counter_more_than = false
#[test]
fn fn_counter_status_t_6() {
    let rule = Rule::new(r"\w", MatchRequirement::MustBeFound)
        .counter_more_than(4)
        .extend([Rule::new(r"\w", MatchRequirement::MustBeFound)]);
    let mut captures = Rule::find_captures(&rule, &"a b c");
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut captures),
        NextStep::Error(Some(HashMap::from([(DEFAULT_CAPTURE.into(), "a".into())])))
    );
}

/// true,true, MustBeFound, counter_more_than = false
#[test]
fn fn_counter_status_t_7() {
    let rule = Rule::new(r"\w", MatchRequirement::MustNotBeFound)
        .counter_more_than(4)
        .extend([Rule::new(r"\w", MatchRequirement::MustBeFound)]);
    let mut captures = Rule::find_captures(&rule, &"a b c");
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut captures),
        NextStep::Error(Some(HashMap::from([(DEFAULT_CAPTURE.into(), "a".into())])))
    );
}

/// false,true, MustBeFound, counter_more_than = false
#[test]
fn fn_counter_status_t_8() {
    let rule = Rule::new(r"\w", MatchRequirement::MustNotBeFound)
        .counter_more_than(4)
        .extend([Rule::new(r"\w", MatchRequirement::MustBeFound)]);
    let mut captures = Rule::find_captures(&rule, &"");
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut captures),
        NextStep::Error(Some(HashMap::default()))
    );
}

/// false,false, MustBeFound, counter_more_than = false
#[test]
fn fn_counter_status_t_9() {
    let rule = Rule::new(r"\w", MatchRequirement::MustNotBeFound).counter_more_than(4);
    let mut captures = Rule::find_captures(&rule, &"");
    assert_eq!(
        NextStep::next_or_finish_or_error(&rule, &mut captures),
        NextStep::Error(Some(HashMap::default()))
    );
}
