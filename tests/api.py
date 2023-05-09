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


class KeyMissing(BaseError):
    template = "Не найден ключ"
    rules = {r"(?P<aboba>key=\d{4})": It.MustBeFoundHere}
    rules = {r"key=\d{4}-\d{4}-\d{4}": It.MustBeFoundHere}


class CustomError(BaseError):
    template = "Опасность отсутствует повторение"
    rules = {
        r"(\w+?)(.+\1)": It.MustBeFoundHere,
        r"(\w)(.+\1)": It.MustBeFoundHere,
    }

# ==============================================


async def init():
    validator_sample = TemplateValidator(
        [CustomError])
    text_bytes = str(
        "wql;").encode('UTF-8')
    try:
        await validator_sample.validate(text_bytes)

    except BaseError as e:
        print(f"ERROR MESSAGE: '{e.message}'")


# =============================================
loop = asyncio.get_event_loop()
task = loop.create_task(init())
loop.run_until_complete(task)
# ============================================
