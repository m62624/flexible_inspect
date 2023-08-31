use flexible_inspect_rs::prelude::*;
#[test]
fn test_range_from_le_bytes() {
    // number is 10
    let text = [10, 0, 0, 0];

    let cartr_1 = Cartridge::new(
        0,
        "not converted to number",
        [RuleBytes::new(
            r"\x0A\x00\x00\x00|\x0C\x00\x00\x00|\x50\x00\x00\x00|\x5A\x00\x00\x00",
            MatchRequirement::MustBeFound,
        )
        .number_range(0..=2500, ReadMode::FromLeBytes, RangeMode::Any)],
    );

    let validator = TemplateValidator::new([cartr_1]);
    dbg!(validator.validate(text.as_ref()));
}
