import pystval
import asyncio
from pystval import Validator, IfFound


class BaseError(Exception):
    template = ""

    def __init__(self, message: str = None, rules: list[str] = None, **extra):
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


class UsernameFieldMissingError(BaseError):
    template = "Error: username field is missing or invalid (current name : {name})"
    rules = {"rule1"}


class CustomError(BaseError):
    template = "Не разрешенный импорт : {import}"
    rules = [r"(?P<import>import aboba from .+)", r"br br br", r"gh gh gh"]

# ==============================================


validator_html = Validator(
    flags_errors=[CustomError, UsernameFieldMissingError])
# async def init():
#     validator1 = Validator(
#         [CustomError, UsernameFieldMissingError])
#     text_bytes = str("import aboba ").encode('UTF-8')
#     try:
#         await validator1.validate(text_bytes)

#     except BaseError as e:
#         print(f"ERROR MESSAGE: '{e.message}'")

# # =============================================
# loop = asyncio.get_event_loop()
# task = loop.create_task(init())
# loop.run_until_complete(task)
# ============================================
