use flexible_inspect_rs::prelude::*;
#[test]
fn test_range_from_le_bytes() {
    // number is 10
    let text = "hello INCORRECT TOKEN 32103301230";

    let test_valid = Cartridge::new(
        -500,
        "Error? not good",
        [Rule::new(r"I*\d+", MatchRequirement::MustBeFound)
            .all_r_for_any_m()
            .extend([
                Rule::new(r"\d+(?=€)", MatchRequirement::MustBeFound),
                Rule::new(r"I*\d+", MatchRequirement::MustBeFound)
                    .any_r_for_any_m()
                    .extend([]),
            ])],
    );
    let test_valid0 = Cartridge::new(
        -500,
        "Error? not good",
        [Rule::new(r"I*\d+", MatchRequirement::MustBeFound)
            .all_r_for_any_m()
            .extend([
                Rule::new(r"\d+(?=€)", MatchRequirement::MustBeFound),
                Rule::new(r"I*\d+", MatchRequirement::MustBeFound)
                    .any_r_for_any_m()
                    .extend([]),
            ])],
    );
    let test_valid1 = Cartridge::new(
        -500,
        "Error? not good",
        [Rule::new(r"I*\d+", MatchRequirement::MustBeFound)
            .all_r_for_any_m()
            .extend([
                Rule::new(r"\d+(?=€)", MatchRequirement::MustBeFound),
                Rule::new(r"I*\d+", MatchRequirement::MustBeFound)
                    .any_r_for_any_m()
                    .extend([]),
            ])],
    );
    let test_valid2 = Cartridge::new(
        -500,
        "Error? not good",
        [Rule::new(r"I*\d+", MatchRequirement::MustBeFound)
            .all_r_for_any_m()
            .extend([
                Rule::new(r"\d+(?=€)", MatchRequirement::MustBeFound),
                Rule::new(r"I*\d+", MatchRequirement::MustBeFound)
                    .any_r_for_any_m()
                    .extend([]),
            ])],
    );

    let validator_for_simple_text = TemplateValidator::new([test_valid, test_valid0]);
    validator_for_simple_text.validate("DASDASDS");

    validator_for_simple_text.validate("DASDASDS");

    validator_for_simple_text.validate("DASDASDS");

    validator_for_simple_text.validate("DASDASDS");

    let validator_for_CAT = TemplateValidator::new([test_valid1, test_valid2]);
    // dbg!(test_valid);
}
