import pystval
import asyncio
from pystval import TemplateValidator, MatchRequirement,Rule, PystvalError

class CustomError_1(PystvalError):
    message = "my custom error 1"
    rules = [Rule("Linux",MatchRequirement.MustNotBefound)]
    


class CustomError_2(PystvalError):
    message = "my custom error 2"
    rules = [
        Rule("2 Root rule",MatchRequirement.MustBeFound).extend([
        Rule("2 Subrule-1 of Root",MatchRequirement.MustBeFound),
        Rule("^a(?>bc|b)",MatchRequirement.MustBeFound).extend([
            Rule("2 Subrule-1 of Subrule-2",MatchRequirement.MustBeFound),
            Rule("2 Subrule-2 of Subrule-2",MatchRequirement.MustBeFound),
        ]).extend([
            Rule("2 SubSubrule-1 of Subrule-2",MatchRequirement.MustBeFound),
            Rule("2 SubSubrule-2 of Subrule-2",MatchRequirement.MustBeFound),
        ]),
        Rule("2 Subrule-3 of Root",MatchRequirement.MustBeFound),
        Rule("2 Subrule-4 of Root",MatchRequirement.MustBeFound).extend([
            Rule("2 Subrule-1 of Subrule-4",MatchRequirement.MustBeFound),
            Rule("2 Subrule-2 of Subrule-4",MatchRequirement.MustBeFound),
        ])
        ])]

# ==============(ERROR FLAGS)==================


# ============================================


async def init():
    # text = await get_bytes("-- link --")
    with open('Makefile', 'rb') as file:
        text = file.read()
    validator_sample = TemplateValidator([CustomError_1])
    try:
        validator_sample.validate_single_sync(text)
    except PystvalError as e:
        print(f"ERROR VALIDATE: '{e.message}'")


# =============================================
loop = asyncio.get_event_loop()
task = loop.create_task(init())
loop.run_until_complete(task)
# ============================================