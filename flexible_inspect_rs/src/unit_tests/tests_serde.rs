use crate::prelude::*;
#[test]
fn tests_serde_0() -> Result<(), serde_json::Error> {
    let rule_1 = Rule::new(r"\w+", MatchRequirement::MustBeFound).extend([
        Rule::new(r"a", MatchRequirement::MustBeFound),
        Rule::new(r"b", MatchRequirement::MustBeFound),
    ]);
    let rule_2 = Rule::new(r"\w+", MatchRequirement::MustBeFound);

    let cartrdige = Cartridge::new(-128, "test", [rule_1, rule_2]);

    let validator_test = TemplateValidator::new([cartrdige]);
    let _result = validator_test.validate("abc");
    assert_eq!(
        serde_json::to_string(&validator_test)?,
        r###"{"cartridges":[{"root_rule":{"str_with_type":{"DefaultRegex":"SYSTEM_ROOT_RULE"},"general_modifiers":{"requirement":"MustBeFound","counter":null,"mod_match":"AllRulesForAtLeastOneMatch"},"subrules":{"simple_rules":{"all_rules":[{"str_with_type":{"DefaultRegex":"\\w+"},"general_modifiers":{"requirement":"MustBeFound","counter":null,"mod_match":"AllRulesForAllMatches"},"subrules":{"simple_rules":{"all_rules":[{"str_with_type":{"DefaultRegex":"a"},"general_modifiers":{"requirement":"MustBeFound","counter":null,"mod_match":"AllRulesForAllMatches"},"subrules":null},{"str_with_type":{"DefaultRegex":"b"},"general_modifiers":{"requirement":"MustBeFound","counter":null,"mod_match":"AllRulesForAllMatches"},"subrules":null}],"regex_set":["a","b"]},"complex_rules":null}},{"str_with_type":{"DefaultRegex":"\\w+"},"general_modifiers":{"requirement":"MustBeFound","counter":null,"mod_match":"AllRulesForAllMatches"},"subrules":null}],"regex_set":["\\w+","\\w+"]},"complex_rules":null}},"id":-128,"message":"test"}],"_phantom":null}"###
    );
    Ok(())
}

#[test]
fn tests_serde_1() -> Result<(), serde_json::Error> {
    let rule_1 = Rule::new(r"\w+", MatchRequirement::MustBeFound).extend(vec![
        Rule::new(r"a", MatchRequirement::MustBeFound),
        Rule::new(r"b", MatchRequirement::MustBeFound),
    ]);
    let rule_2 = Rule::new(r"\w+", MatchRequirement::MustBeFound);

    let cartrdige = Cartridge::new(-128, "test", vec![rule_1, rule_2]);

    let validator_orig: TemplateValidator<Vec<Cartridge<Rule>>, &str> =
        TemplateValidator::new(vec![cartrdige]);
    let _result = validator_orig.validate("abc");
    let validator_serde = serde_json::from_str::<TemplateValidator<Vec<Cartridge<Rule>>, &str>>(
        serde_json::to_string(&validator_orig)?.as_str(),
    )?;
    assert_eq!(validator_orig, validator_serde);
    Ok(())
}

#[test]
fn tests_serde_2() -> Result<(), serde_json::Error> {
    let rule_1 = RuleBytes::new(r"\w+", MatchRequirement::MustBeFound).extend([
        RuleBytes::new(r"a", MatchRequirement::MustBeFound),
        RuleBytes::new(r"b", MatchRequirement::MustBeFound),
    ]);
    let rule_2 = RuleBytes::new(r"\w+", MatchRequirement::MustBeFound);

    let cartrdige = Cartridge::new(-128, "test", [rule_1, rule_2]);

    let validator_test = TemplateValidator::new([cartrdige]);
    let _result = validator_test.validate("abc".as_bytes());
    assert_eq!(
        serde_json::to_string(&validator_test)?,
        r###"{"cartridges":[{"root_rule":{"str_bytes":"SYSTEM_ROOT_RULE","general_modifiers":{"requirement":"MustBeFound","counter":null,"mod_match":"AllRulesForAtLeastOneMatch"},"subrules_bytes":{"all_rules":[{"str_bytes":"\\w+","general_modifiers":{"requirement":"MustBeFound","counter":null,"mod_match":"AllRulesForAllMatches"},"subrules_bytes":{"all_rules":[{"str_bytes":"a","general_modifiers":{"requirement":"MustBeFound","counter":null,"mod_match":"AllRulesForAllMatches"},"subrules_bytes":null},{"str_bytes":"b","general_modifiers":{"requirement":"MustBeFound","counter":null,"mod_match":"AllRulesForAllMatches"},"subrules_bytes":null}],"regex_set":["a","b"]}},{"str_bytes":"\\w+","general_modifiers":{"requirement":"MustBeFound","counter":null,"mod_match":"AllRulesForAllMatches"},"subrules_bytes":null}],"regex_set":["\\w+","\\w+"]}},"id":-128,"message":"test"}],"_phantom":null}"###
    );
    Ok(())
}

#[test]
fn tests_serde_3() -> Result<(), serde_json::Error> {
    let rule_1 = RuleBytes::new(r"\w+", MatchRequirement::MustBeFound).extend(vec![
        RuleBytes::new(r"a", MatchRequirement::MustBeFound),
        RuleBytes::new(r"b", MatchRequirement::MustBeFound),
    ]);
    let rule_2 = RuleBytes::new(r"\w+", MatchRequirement::MustBeFound);

    let cartrdige = Cartridge::new(-128, "test", vec![rule_1, rule_2]);

    let validator_orig: TemplateValidator<Vec<Cartridge<RuleBytes>>, &[u8]> =
        TemplateValidator::new(vec![cartrdige]);
    let _result = validator_orig.validate("abc".as_bytes());
    let validator_serde = serde_json::from_str::<
        TemplateValidator<Vec<Cartridge<RuleBytes>>, &[u8]>,
    >(serde_json::to_string(&validator_orig)?.as_str())?;
    assert_eq!(validator_orig, validator_serde);
    Ok(())
}
