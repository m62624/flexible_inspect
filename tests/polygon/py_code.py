import asyncio
from pystval import Rule, MatchRequirement, TemplateValidator, PystvalException


class ErrorAboba(PystvalException):
    message = "Not found aboba"
    rules = [
        Rule("aboba", MatchRequirement.MustBeFound)
    ]


class ErrorNumber(PystvalException):
    message = "found number {n}"
    rules = [
        Rule("(?P<n>\d+)", MatchRequirement.MustNotBeFound),
    ]


class CheckExtraBlank(PystvalException):
    message = "Extra blank {e}"
    rules = [
        Rule(r"(?P<e>\[.+\])", MatchRequirement.MustBeFound).extend([
            Rule(r"(?P<e>\d+)", MatchRequirement.MustNotBeFound)
        ])
    ]


async def init(delay, text):
    await asyncio.sleep(delay)
    validator = TemplateValidator([CheckExtraBlank])
    try:
        await validator.async_validate(text)
    except PystvalException as e:
        print(f"{delay}, {e.report}")


async def main():
    tasks = [
        asyncio.create_task(init(1, b"[SA] [WQ] [XA] [123]")),
    ]

    await asyncio.gather(*tasks)

loop = asyncio.get_event_loop()
task = loop.create_task(main())
loop.run_until_complete(task)
