import pystval
import asyncio
from pystval import TemplateValidator, MatchRequirement


class PystvalErrorMeta(type):
    def __new__(cls, name, bases, attrs):
        if name == 'PystvalError':
            attrs['allow_default_values'] = True
        else:
            if 'message' not in attrs or not attrs['message']:
                raise NotImplementedError(
                    "Subclasses must provide a non-empty 'message' attribute.")
            if 'rules' not in attrs or not isinstance(attrs['rules'], dict):
                raise NotImplementedError(
                    "Subclasses must provide a 'rules' attribute of type 'dict'.")
        return super().__new__(cls, name, bases, attrs)


class PystvalError(Exception, metaclass=PystvalErrorMeta):

    def __init__(self, message: str = None, rules: dict[str, enumerate] = None, **extra):
        self._extra = extra
        self._rules = rules
        if message is None:
            self._message = self.message.format(**extra)

    @property
    def message(self):
        return self._message

    @property
    def extra(self):
        return self._extra

    @property
    def rules(self):
        return self._rules


class FindSomething(PystvalError):
    message = "Gotcha : {OS}"
    rules = {
        r"""(?P<OS>Linux)""": MatchRequirement.NotToBeFoundHere,
    }

# # ============================================


async def init():
    # text = await get_bytes("-- link --")
    with open('Makefile', 'rb') as file:
        text = file.read()
    validator_sample = TemplateValidator(flags=[FindSomething])
    try:
        await validator_sample.validate(text)
    except PystvalError as e:
        print(f"ERROR VALIDATE: '{e.extra}'")


# # # =============================================
loop = asyncio.get_event_loop()
task = loop.create_task(init())
loop.run_until_complete(task)
# ============================================
