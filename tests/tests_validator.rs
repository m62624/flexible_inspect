use pystval::prelude::*;
use std::fs::read_to_string;

const DOC_HTML_1: &str = "tests/files/doc_html_1.html";
const DOC_HTML_1_WITH_ERROR: &str = "tests/files/doc_html_1_error.html";
/// Проверяем что есть input password в html документе
#[test]
fn test_validator_t_0() {
    let file_html = read_to_string(DOC_HTML_1).unwrap();
    let check_password = Cartridge::new(
        404,
        "html document error, password section missing",
        [
            Rule::new("label.+\n.+", MatchRequirement::MustBeFound).extend([Rule::new(
                r#"<input type="password" id="password".+"#,
                MatchRequirement::MustBeFound,
            )]),
        ],
    )
    .mode_at_least_one_rule_for_at_least_one_match();

    let validator_for_html = TemplateValidator::new([check_password]);
    assert!(validator_for_html.validate(file_html.as_str()).is_ok());
}

#[test]
fn test_validator_t_1() {
    let file_html = read_to_string(DOC_HTML_1_WITH_ERROR).unwrap();
    let check_password = Cartridge::new(
        404,
        "html document error, password section missing",
        [
            Rule::new("label.+\n.+", MatchRequirement::MustBeFound).extend([Rule::new(
                r#"<input type="password" id="password".+"#,
                MatchRequirement::MustBeFound,
            )]),
        ],
    )
    .mode_at_least_one_rule_for_at_least_one_match();
    let validator_for_html = TemplateValidator::new([check_password]);
    assert!(validator_for_html.validate(file_html.as_str()).is_err());
}

#[test]
fn test_validator_t_2() {
    let file_html = read_to_string(DOC_HTML_1).unwrap();
    let check_password = Cartridge::new(
        404,
        "html document error, password section missing",
        [
            Rule::new("label.+\n.+", MatchRequirement::MustBeFound).extend([Rule::new(
                r#"<input type="password" id="password".+"#,
                MatchRequirement::MustBeFound,
            )]),
        ],
    )
    .mode_at_least_one_rule_for_at_least_one_match();

    let validator_for_html = TemplateValidator::new([check_password]);

    async_std::task::block_on(async {
        assert!(validator_for_html
            .async_validate(file_html.as_str())
            .await
            .is_ok())
    });
}
