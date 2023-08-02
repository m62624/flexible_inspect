use crate::prelude::*;
#[test]
fn tests_serde() -> Result<(), serde_json::Error> {
    let rule_1 = Rule::new(r"\w+", MatchRequirement::MustBeFound).extend([
        Rule::new(r"a", MatchRequirement::MustBeFound),
        Rule::new(r"b", MatchRequirement::MustBeFound),
    ]);
    let rule_2 = Rule::new(r"\w+", MatchRequirement::MustBeFound);

    let cartrdige = Cartridge::new(-128, "test", [rule_1, rule_2]);

    let validator_test = TemplateValidator::new([cartrdige]);
    let _result = validator_test.validate("abc");
    let x = serde_json::to_string(&validator_test)?;
    dbg!(x);
    Ok(())
}
