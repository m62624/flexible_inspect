use super::*;
use crate::template_validator::TemplateValidator;

#[test]
pub fn test_number_range_t_0() {
    let text = b"1 2 3 4 5 6 2 sad flq wqp  8r 9 10";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [
            RuleBytes::new(r"\d+", MatchRequirement::MustBeFound).number_range(
                1..=20,
                ReadMode::FromUtf8,
                RangeMode::All,
            ),
        ],
    )]);

    assert!(validator_numbers.validate(text.as_slice()).is_ok());
}

#[test]
pub fn test_number_range_t_1() {
    let text = b"1 2 3 4 5 6 2 9 10";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [
            RuleBytes::new(r"\d+", MatchRequirement::MustBeFound).number_range(
                1..=4,
                ReadMode::FromUtf8,
                RangeMode::All,
            ),
        ],
    )]);

    assert!(validator_numbers.validate(text.as_slice()).is_err());
}

#[test]
pub fn test_number_range_t_2() {
    let text = b"1 2 3";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [
            RuleBytes::new(r"\d+", MatchRequirement::MustBeFound).number_range(
                1..=4,
                ReadMode::FromUtf8,
                RangeMode::Exactly(3),
            ),
        ],
    )]);

    assert!(validator_numbers.validate(text.as_slice()).is_ok());
}

#[test]
pub fn test_number_range_t_3() {
    let text = b"1 1 4 5 6 7 8";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)
            .save_duplicates()
            .number_range(1..=4, ReadMode::FromUtf8, RangeMode::Exactly(2))],
    )]);

    assert!(validator_numbers.validate(text.as_slice()).is_err());
}

#[test]
pub fn test_number_range_t_4() {
    let text = b"1 1 4 7 8";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)
            .save_duplicates()
            .number_range(5..=7, ReadMode::FromUtf8, RangeMode::Any)],
    )]);

    assert!(validator_numbers.validate(text.as_slice()).is_ok());
}

#[test]
pub fn test_number_range_t_5() {
    let text = b"1 1 4 7 8";
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)
            .save_duplicates()
            .number_range(10..=11, ReadMode::FromUtf8, RangeMode::Any)],
    )]);

    assert!(validator_numbers.validate(text.as_slice()).is_err());
}

#[test]
pub fn test_number_range_t_6() {
    let text = [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
    let validator_numbers = TemplateValidator::new([Cartridge::new(
        0,
        "error message from cartridge 1",
        [RuleBytes::new(
            r"\x00\x00\x00\x01|\x00\x00\x00\x02|\x00\x00\x00\x03",
            MatchRequirement::MustBeFound,
        )
        .number_range(1..=3, ReadMode::FromBeBytes, RangeMode::Any)],
    )]);

    assert!(validator_numbers.validate(text.as_slice()).is_ok());
}
