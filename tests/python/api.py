import asyncio
import pystval
from pystval import TemplateValidator, TemplateValidatorBytes, Rule, RuleBytes, Cartridge, CartridgeBytes, MatchRequirement, PystvalException

rule_1 = Rule(r"\d+", MatchRequirement.MustBeFound)
rule_1 = Rule(r"ABC", MatchRequirement.MustNotBeFound)

rule_bytes_1 = RuleBytes(r"\d+", MatchRequirement.MustBeFound)
rule_bytes_1 = RuleBytes(r"\w+", MatchRequirement.MustNotBeFound)

cartridge_1 = Cartridge(1, "message from cartridge str ", [rule_1, rule_1])
cartridge_bytes_1 = CartridgeBytes(1, "message from cartridge bytes", [
                                   rule_bytes_1, rule_bytes_1])

validator_text = TemplateValidator([cartridge_1])
validator_bytes = TemplateValidatorBytes([cartridge_bytes_1])

text_str = "12322312 ABC"
text_bytes = b"12322312"


async def init():
    result = await validator_text.async_validate(text_str)
    if result is None:
        print("OK")
    else:
        for error in result:
            try:
                raise error
            except Exception:
                print(error.get_code())

loop = asyncio.get_event_loop()
task = loop.create_task(init())
loop.run_until_complete(task)
