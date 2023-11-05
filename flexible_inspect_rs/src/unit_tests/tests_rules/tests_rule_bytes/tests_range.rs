use super::*;
use crate::template_validator::TemplateValidator;

#[test]
pub fn test_number_range_t_0() {
    let text_bytes = b"1 2 3 4 5 6 2 sad flq wqp  8r 9 10";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error number",
        [
            RuleBytes::new(r"\d+", MatchRequirement::MustBeFound).number_range(
                1..=20,
                ReadMode::FromUtf8,
                RangeMode::All,
            ),
        ],
    )]);
    assert!(validator_numbers.validate(text_bytes.as_slice()).is_ok());
}

#[test]
pub fn test_number_range_t_1() {
    let text_bytes = b"1 2 3 4 5 6 2 9 10";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error number",
        [
            RuleBytes::new(r"\d+", MatchRequirement::MustBeFound).number_range(
                1..=4,
                ReadMode::FromUtf8,
                RangeMode::All,
            ),
        ],
    )]);
    assert!(validator_numbers.validate(text_bytes.as_slice()).is_err());
}