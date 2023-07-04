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
        Rule("(?P<n>\d+)", MatchRequirement.MustNotBefound)
    ]

async def init(delay,text):
    await asyncio.sleep(delay)
    validator = TemplateValidator([ErrorAboba, ErrorNumber])
    try:
        await validator.async_validate(text)
    except PystvalException as e:
        print(f"{delay}, {e.report}")

async def main():
    tasks = [
        asyncio.create_task(init(50,b"aboba")),  
        asyncio.create_task(init(4,b"bob")),
        asyncio.create_task(init(6,b"HEY")),
    ]

    await asyncio.gather(*tasks)

loop = asyncio.get_event_loop()
task = loop.create_task(main())
loop.run_until_complete(task)
