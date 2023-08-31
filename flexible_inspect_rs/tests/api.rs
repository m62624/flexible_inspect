use flexible_inspect_rs::prelude::*;
#[test]
fn main_rust() {
    let text = b"0x0E 0x0F 0x10 0x11 0x12 0x13 0x14 0x15 0x16 0x17 0x18";

    let cartr_1 = Cartridge::new(
        0,
        "not converted to number",
        [
            RuleBytes::new(r"0x..", MatchRequirement::MustBeFound)
            // .number_range(
            //     0..=10000,
            //     ReadMode::FromLeBytes,
            //     RangeMode::Any,
            // ),
        ],
    );

    // dbg!(&cartr_1);

    let validator = TemplateValidator::new([cartr_1]);
    dbg!(validator.validate(text.as_ref()));
}
