use flexible_inspect_rs::prelude::*;

#[async_std::test]
async fn async_validate_t_0() {
    let cartridge_1 = Cartridge::new(
        0,
        "error message from cartridge 1",
        [Rule::new(r"\d+", MatchRequirement::MustBeFound)],
    );
    let cartridge_2 = Cartridge::new(
        1,
        "error message from cartridge 2",
        [Rule::new(r"ABC", MatchRequirement::MustNotBeFound)],
    );

    let validator = TemplateValidator::new([cartridge_1, cartridge_2]);


    assert!(validator.async_validate("1234").await.is_ok());
}

#[test]
fn validate_t_0() {
    let cartridge_1 = Cartridge::new(
        0,
        "error message from cartridge 1",
        [Rule::new(r"\d+", MatchRequirement::MustBeFound)],
    );
    let cartridge_2 = Cartridge::new(
        1,
        "error message from cartridge 2",
        [Rule::new(r"ABC", MatchRequirement::MustNotBeFound)],
    );

    let validator = TemplateValidator::new([cartridge_1, cartridge_2]);

    assert!(validator.validate("1234").is_ok());
}
