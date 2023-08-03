const { MatchRequirement, Rule, RuleBytes } = require('../../pkg');

let rule_root = Rule.start_build("\d+", MatchRequirement).extend([
    Rule.start_build("A", MatchRequirement).mode_all_rules_for_at_least_one_match().finish_build(),
    Rule.start_build("B", MatchRequirement).mode_at_least_one_rule_for_all_matches().finish_build(),
    Rule.start_build("C", MatchRequirement).mode_at_least_one_rule_for_at_least_one_match().finish_build(),
    Rule.start_build("\d+(?=\d+)", MatchRequirement).finish_build(),
]).finish_build();

console.log(rule_root);
