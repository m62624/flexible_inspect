use super::*;
#[test]
fn test_new_t_0() -> PyResult<()> {
    let cartridge: Cartridge<RuleBytes> = PyCartridgeBytes::new(
        0,
        "message_0".into(),
        vec![PyRuleBytes::new(
            r"\w+".into(),
            PyMatchRequeriment::MustBeFound,
        )],
    )?
    .try_into()?;

    assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![RuleBytes::new(r"\w+", MatchRequirement::MustBeFound)],
        ),
    );
    Ok(())
}

#[test]
fn test_mode_match_t_0() -> PyResult<()> {
    let cartridge: Cartridge<RuleBytes> = PyCartridgeBytes::new(
        0,
        "message_0".into(),
        vec![PyRuleBytes::new(
            r"\w+".into(),
            PyMatchRequeriment::MustBeFound,
        )],
    )?
    .any_r_for_any_m()?
    .try_into()?;

    assert_eq!(
        cartridge,
        Cartridge::new(
            0,
            "message_0",
            vec![RuleBytes::new(r"\w+", MatchRequirement::MustBeFound)],
        )
        .any_r_for_any_m(),
    );
    Ok(())
}
