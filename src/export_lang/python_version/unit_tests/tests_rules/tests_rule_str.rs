use super::*;

#[test]
fn fn_new_t_0() {
    PyRule::new(r"x".into(), PyMatchRequirement::MustBeFound);
}

#[test]
fn fn_new_t_1() {
    dbg!(PyRule::new(r"x".into(), PyMatchRequirement::MustNotBeFound));
}

#[test]
#[should_panic]
fn fn_new_e_0() {
    PyRule::new(r"\x".into(), PyMatchRequirement::MustBeFound);
}
