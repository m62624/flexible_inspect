const { Rule, RuleBytes, MatchRequirement, Cartridge, CartridgeBytes, TemplateValidator, TemplateValidatorBytes, init_logger, LogLevel } = require('../pkg');


async function main_test() {
    let base_check_text = new Cartridge(0, "ERROR FROM CARTRIDGE_0", [
        new Rule(/\d{3}-\d{2}-\d{4}/, MatchRequirement.MustNotBeFound).finish_build(),
    ]).finish_build();

    let external_check_text = new Cartridge(1, "ERROR FROM CARTRIDGE_1", [
        new Rule(/\./, MatchRequirement.MustNotBeFound).finish_build(),
    ]).finish_build();

    let validator_text = new TemplateValidator([base_check_text, external_check_text]);
    const result = validator_text.validate("123 ABC .[sdad]");
    if (result != undefined) {
        console.log("Error: " + result.next().get_message());
    }
}

init_logger(LogLevel.DEBUG);
main_test();