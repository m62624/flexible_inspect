use crate::export_lang::python_version::rules::rule_str::PyRule;
use crate::export_lang::python_version::rules::PyMatchRequirement;

#[test]
fn rule_new_t_0() {
    PyRule::new(r"x".into(), PyMatchRequirement::MustBeFound);
}

#[test]
fn rule_new_t_1() {
    dbg!(PyRule::new(r"x".into(), PyMatchRequirement::MustNotBeFound));
}

#[test]
#[should_panic]
fn rule_new_e_0() {
    PyRule::new(r"\x".into(), PyMatchRequirement::MustBeFound);
}
