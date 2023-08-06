use pystval::prelude::*;

#[async_std::test]
async fn main_test() {
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
