import pystval
import asyncio
from pystval import TemplateValidator, IfFound


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


class UsernameFieldMissingError(BaseError):
    template = "messsage message message"
    rules = {r"rule1": IfFound.AllRight,
             r"rule2": IfFound.RaiseError,
             r"rule3": IfFound.RaiseError,
             r"rule4": IfFound.AllRight, }


class CustomError(BaseError):
    template = "messsage message message"
    rules = {r"rule5##": IfFound.RaiseError,
             r"(\w+?)(.+\1)": IfFound.AllRight,
             r"rule7##": IfFound.RaiseError,
             r"rule8##": IfFound.AllRight,
             }

# ==============================================


try:
    validator_html = TemplateValidator(
        flags=[UsernameFieldMissingError, CustomError])
except Exception as e:
    print(f"Произошла ошибка: {e}")
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
