const { Rule, RuleBytes, MatchRequirement, Cartridge, CartridgeBytes } = require('../pkg');

let rule_root_1 = new Rule("ROOT_1", MatchRequirement.MustBeFound).extend([
    new Rule("A", MatchRequirement.MustBeFound).finish_build(),
    new Rule("B", MatchRequirement.MustBeFound).finish_build(),
    new Rule("C", MatchRequirement.MustBeFound).finish_build(),
]).counter_less_than(5).mode_all_rules_for_at_least_one_match().finish_build();

let rule_root_2 = new Rule("ROOT_2", MatchRequirement.MustBeFound).extend([
    new Rule("D", MatchRequirement.MustBeFound).finish_build(),
    new Rule("F", MatchRequirement.MustBeFound).finish_build(),
    new Rule("E", MatchRequirement.MustBeFound).finish_build(),
]).counter_less_than(5).mode_all_rules_for_at_least_one_match().finish_build();


let rule_root_1_bytes = new RuleBytes("ROOT_2", MatchRequirement.MustBeFound).extend([
    new RuleBytes("D", MatchRequirement.MustBeFound).finish_build(),
    new RuleBytes("F", MatchRequirement.MustBeFound).finish_build(),
    new RuleBytes("E", MatchRequirement.MustBeFound).finish_build(),
]).counter_less_than(5).mode_all_rules_for_at_least_one_match().finish_build();

let cartr = new Cartridge(1, "main_cartridge", [rule_root_1_bytes, rule_root_2]).finish_build()
console.log(cartr);