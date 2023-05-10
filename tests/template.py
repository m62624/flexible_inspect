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
        r"""(?s)<img.+?id="avatar"\s?.+?\s?>""": It.MustBeFoundHere,
    }


class MissingElementFollowers(BaseError):
    template = ":: The `followers` element was not found"
    rules = {
        r"""(?s)<a.+?id="followers"\s?.+?>.+?</a>""": It.MustBeFoundHere,
    }


class MissingElementFollowing(BaseError):
    template = ":: The `following` element was not found"
    rules = {
        r"""(?s)<a.+?id="following"\s?.+?>.+?</a>""": It.MustBeFoundHere,
    }


class MissingElementFriends(BaseError):
    template = ":: The `friends` element was not found"
    rules = {
        r"""(?s)<a.+?id="friends"\s?.+?>.+?</a>""": It.MustBeFoundHere,
    }


class UniqueDefaultElement(BaseError):
    template = ":: More than one element is found : {one_element}"
    rules = {
        r"""(?s)(?P<one_element><a.+?id="followers"\s?.+?>.+?</a>)(?=\s*?.*?\k<one_element>)""": It.NotToBeFoundHere,
        r"""(?s)(?P<one_element><a.+?id="following"\s?.+?>.+?</a>)(?=\s*?.*?\k<one_element>)""": It.NotToBeFoundHere,
        r"""(?s)(?P<one_element>(?s)<a.+?id="friends"\s?.+?>.+?</a>)(?=\s*?.*?\k<one_element>)""": It.NotToBeFoundHere,
    }
# ============================================


async def init():
    # text = await get_bytes("-- link --")
    with open('tests/tmp/body.html', 'rb') as file:
        text = file.read()
    validator_sample = TemplateValidator(
        flags=[MissingElementAvatar,
               MissingElementFollowers,
               MissingElementFollowing,
               MissingElementFriends,
               UniqueDefaultElement])
    try:
        await validator_sample.validate(text)
    except BaseError as e:
        print(f"ERROR VALIDATE: '{e.message}'")


# =============================================
loop = asyncio.get_event_loop()
task = loop.create_task(init())
loop.run_until_complete(task)
# ============================================
