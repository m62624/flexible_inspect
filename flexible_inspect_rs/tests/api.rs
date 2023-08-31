use flexible_inspect_rs::prelude::*;
#[test]
fn main_rust() {
    let text = b"0xFF, 0xFF, 0xFF, 0xFF";

    let cartr_1 = Cartridge::new(
        0,
        "x",
        [
            RuleBytes::new(r"0x", MatchRequirement::MustBeFound).number_range(
                0..=255,
                ReadMode::FromLeBytes,
                RangeMode::Any,
            ),
        ],
    );

    dbg!(&cartr_1);

    let validator = TemplateValidator::new([cartr_1]);
    dbg!(validator.validate(text.as_ref()));
}
