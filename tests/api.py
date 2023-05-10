import httpx
import pystval
import asyncio
from pystval import TemplateValidator, It


class BaseError(Exception):
    template = ""

    def __init__(self, message: str = None, rules: dict[str, enumerate] = None, **extra):
        self._extra = extra
        self._rules = rules
        if message is None:
            self._message = self.template.format(**extra)
        else:
            self._message = message.format(**extra)

    @property
    def message(self):
        return self._message

    @property
    def extra(self):
        return self._extra

    @property
    def rules(self):
        return self._rules

# =============================================


async def get_bytes(url):
    async with httpx.AsyncClient() as client:
        response = await client.get(url)
        response.raise_for_status()
        return response.content

# ==============(ERROR FLAGS)==================


class MissingElementAvatar(BaseError):
    template = ":: The `avatar` element was not found"
    rules = {
        r"""<im id="avatar"\s?.+?\s?>""": It.MustBeFoundHere,
    }


class MissingElementFollowers(BaseError):
    template = ":: The `Followers` element was not found"
    rules = {
        r"""<a id="followers"\s?.+?>.+?</a>""": It.MustBeFoundHere,
    }

# ============================================


async def init():
    text = await get_bytes("https://test-cdn.yourbandy.com/profile_templates/b7888914-6356-4904-8950-774e3057e034_profile_template_28.html")
    validator_sample = TemplateValidator(
        flags=[MissingElementAvatar])
    try:
        await validator_sample.validate(text)
    except BaseError as e:
        print(f"ERROR VALIDATE: '{e.message}'")


# =============================================
loop = asyncio.get_event_loop()
task = loop.create_task(init())
loop.run_until_complete(task)
# ============================================
