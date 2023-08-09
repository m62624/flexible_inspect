use super::*;

#[test]
fn test_new_t_0() {
    let cartridge: Cartridge<Rule> = PyCartridge::new(
        0,
        "message_0".into(),
        vec![PyRule::new(r"\w+".into(), PyMatchRequeriment::MustBeFound)],
    )
    .into();

    assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![Rule::new(r"\w+", MatchRequirement::MustBeFound)],
        ),
    )
}
