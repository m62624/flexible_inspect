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


class UsernameFieldMissingError(BaseError):
    template = "messsage message message"
    rules = {r"rule1": It.MustBeFoundHere,
             r"rule2": It.NotToBeFoundHere,
             r"rule3": It.NotToBeFoundHere,
             r"rule4": It.MustBeFoundHere, }


class CustomError(BaseError):
    template = "messsage message message"
    rules = {r"rule5##": It.NotToBeFoundHere,
             r"(\w+?)(.+\1)": It.MustBeFoundHere,
             r"rule7##": It.NotToBeFoundHere,
             r"rule8##": It.MustBeFoundHere,
             }

# ==============================================


# try:
#     validator_html = TemplateValidator(
#         flags=[UsernameFieldMissingError, CustomError])
# except Exception as e:
#     print(f"Произошла ошибка: {e}")
async def init():
    validator_sample = TemplateValidator(
        [CustomError, UsernameFieldMissingError])
    text_bytes = str("rule1 qlwlw rule7###").encode('UTF-8')
    try:
        await validator_sample.validate(text_bytes)

    except BaseError as e:
        print(f"ERROR MESSAGE: '{e.message}'")

# =============================================
loop = asyncio.get_event_loop()
task = loop.create_task(init())
loop.run_until_complete(task)
# ============================================
