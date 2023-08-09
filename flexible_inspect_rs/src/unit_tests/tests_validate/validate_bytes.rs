use super::*;

#[async_std::test]
async fn async_validate_t_0() {
    let cartridge_1 = Cartridge::new(
        0,
        "error message from cartridge 1",
        [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)],
    );
    let cartridge_2 = Cartridge::new(
        1,
        "error message from cartridge 2",
        [RuleBytes::new(r"ABC", MatchRequirement::MustNotBeFound)],
    );

    let validator = TemplateValidator::new([cartridge_1, cartridge_2]);

    assert!(validator.async_validate("1234".as_bytes()).await.is_ok());
}

#[async_std::test]
async fn async_validate_t_1() {
    let cartridge_1 = Cartridge::new(
        0,
        "error message from cartridge 1",
        [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)],
    );
    let cartridge_2 = Cartridge::new(
        1,
        "error message from cartridge 2",
        [RuleBytes::new(r"ABC", MatchRequirement::MustNotBeFound)],
    );

    let validator = TemplateValidator::new([cartridge_1, cartridge_2]);

    assert!(validator.async_validate("ABC".as_bytes()).await.is_err());
}

#[test]
fn validate_t_0() {
    let cartridge_1 = Cartridge::new(
        0,
        "error message from cartridge 1",
        [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)],
    );
    let cartridge_2 = Cartridge::new(
        1,
        "error message from cartridge 2",
        [RuleBytes::new(r"ABC", MatchRequirement::MustNotBeFound)],
    );

    let validator = TemplateValidator::new([cartridge_1, cartridge_2]);

    assert!(validator.validate("1234".as_bytes()).is_ok());
}

#[test]
fn validate_t_1() {
    let cartridge_1 = Cartridge::new(
        0,
        "error message from cartridge 1",
        [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)],
    );
    let cartridge_2 = Cartridge::new(
        1,
        "error message from cartridge 2",
        [RuleBytes::new(r"ABC", MatchRequirement::MustNotBeFound)],
    );

    let validator = TemplateValidator::new([cartridge_1, cartridge_2]);

    assert!(validator.validate("ABC".as_bytes()).is_err());
}

#[cfg(feature = "export_to_other_languages")]
mod export_to_other_languages {
    use super::*;

    #[async_std::test]
    async fn exp_async_validate_t_0() {
        let cartridge_1 = Cartridge::new(
            0,
            "error message from cartridge 1",
            [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)],
        );
        let cartridge_2 = Cartridge::new(
            1,
            "error message from cartridge 2",
            [RuleBytes::new(r"ABC", MatchRequirement::MustNotBeFound)],
        );

        let validator = TemplateValidator::new([cartridge_1, cartridge_2]);
        let text: Arc<[u8]> = Arc::from("123".as_bytes());
        assert!(validator.async_validate(Arc::clone(&text)).await.is_ok());
    }

    #[async_std::test]
    async fn exp_async_validate_t_1() {
        let cartridge_1 = Cartridge::new(
            0,
            "error message from cartridge 1",
            [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)],
        );
        let cartridge_2 = Cartridge::new(
            1,
            "error message from cartridge 2",
            [RuleBytes::new(r"ABC", MatchRequirement::MustNotBeFound)],
        );

        let validator = TemplateValidator::new([cartridge_1, cartridge_2]);
        let text: Arc<[u8]> = Arc::from("ABC".as_bytes());
        assert!(validator.async_validate(text).await.is_err());
    }

    #[test]
    fn exp_validate_t_0() {
        let cartridge_1 = Cartridge::new(
            0,
            "error message from cartridge 1",
            [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)],
        );
        let cartridge_2 = Cartridge::new(
            1,
            "error message from cartridge 2",
            [RuleBytes::new(r"ABC", MatchRequirement::MustNotBeFound)],
        );

        let validator = TemplateValidator::new([cartridge_1, cartridge_2]);
        let text: Arc<[u8]> = Arc::from("123".as_bytes());
        assert!(validator.validate(text).is_ok());
    }

    #[test]
    fn exp_validate_t_1() {
        let cartridge_1 = Cartridge::new(
            0,
            "error message from cartridge 1",
            [RuleBytes::new(r"\d+", MatchRequirement::MustBeFound)],
        );
        let cartridge_2 = Cartridge::new(
            1,
            "error message from cartridge 2",
            [RuleBytes::new(r"ABC", MatchRequirement::MustNotBeFound)],
        );

        let validator = TemplateValidator::new([cartridge_1, cartridge_2]);
        let text: Arc<[u8]> = Arc::from("ABC".as_bytes());
        assert!(validator.validate(text).is_err());
    }
}
