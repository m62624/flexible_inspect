const { MatchRequirement, Rule, RuleBytes } = require('../../pkg');

let rule_root = Rule.start_build("\w+(?=\d+)", MatchRequirement.MustNotBeFound).extend([
    Rule.start_build("A",).mode_all_rules_for_at_least_one_match().finish_build(),
    Rule.start_build("B", MatchRequirement.MustBeFound).mode_at_least_one_rule_for_all_matches().finish_build(),
    Rule.start_build("C", MatchRequirement.MustNotBeFound).mode_at_least_one_rule_for_at_least_one_match().finish_build(),
]).finish_build();

console.log(rule_root);