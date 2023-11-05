use super::*;
use crate::template_validator::TemplateValidator;

#[test]
pub fn test_number_range_t_0() {
    let text = "1 2 3 4 5 6 2 sad flq wqp  8r 9 10";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [Rule::new(r"\d+", MatchRequirement::MustBeFound).number_range(1..=20, RangeMode::All)],
    )]);

    assert!(validator_numbers.validate(text).is_ok());
}

#[test]
pub fn test_number_range_t_1() {
    let text = "1 2 3 4 5 6 2 9 10";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [Rule::new(r"\d+", MatchRequirement::MustBeFound).number_range(1..=4, RangeMode::All)],
    )]);

    assert!(validator_numbers.validate(text).is_err());
}

#[test]
pub fn test_number_range_t_2() {
    let text = "1 2 3";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [Rule::new(r"\d+", MatchRequirement::MustBeFound)
            .number_range(1..=4, RangeMode::Exactly(3))],
    )]);

    assert!(validator_numbers.validate(text).is_ok());
}

#[test]
pub fn test_number_range_t_3() {
    let text = "1 2 3";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [Rule::new(r"\d+", MatchRequirement::MustBeFound)
            .number_range(1..=4, RangeMode::Exactly(100))],
    )]);

    dbg!(validator_numbers.validate(text));
}
