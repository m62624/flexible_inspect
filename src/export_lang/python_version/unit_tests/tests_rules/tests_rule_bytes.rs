use super::*;

#[test]
fn fn_new_t_0() {
    dbg!(PyRuleBytes::new(
        r"x".into(),
        PyMatchRequirement::MustBeFound
    ));
}

#[test]
fn fn_new_t_1() {
    dbg!(PyRuleBytes::new(
        r"x".into(),
        PyMatchRequirement::MustNotBeFound
    ));
}

#[test]
#[should_panic]
fn fn_new_e_0() {
    PyRuleBytes::new(r"\x".into(), PyMatchRequirement::MustBeFound);
}
