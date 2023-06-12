import pystval
import asyncio
from pystval import TemplateValidator, It, throw_base_error,BaseErrorWrapper

class check_error(BaseErrorWrapper):
    pass


# class BaseError(Exception):
#     template = ""

#     def __init__(self, message: str = None, rules: dict[str, enumerate] = None, **extra):
#         self._extra = extra
#         self._rules = rules
#         if message is None:
#             self._message = self.template.format(**extra)
#         else:
#             self._message = message.format(**extra)

#     @property
#     def message(self):
#         return self._message

#     @property
#     def extra(self):
#         return self._extra

#     @property
#     def rules(self):
#         return self._rules

# =============================================


# async def get_bytes(url):
#     async with httpx.AsyncClient() as client:
#         response = await client.get(url)
#         response.raise_for_status()
#         return response.content

# # ==============(ERROR FLAGS)==================


# class MissingElementAvatar(BaseErrorWrapper):
#     template = ":: The `avatar` element was not found"
#     rules = {
#         r"""(?s)<img.+?id="avatar"\s?.+?\s?>""": It.MustBeFoundHere,
#     }

# # ============================================


# async def init():
#     # text = await get_bytes("-- link --")
#     with open('tests/tmp/body.html', 'rb') as file:
#         text = file.read()
#     validator_sample = TemplateValidator(
#         flags=[MissingElementAvatar]),
#     try:
#         await validator_sample.validate(text)
#     except BaseError as e:
#         print(f"ERROR VALIDATE: '{e.message}'")


# # =============================================
# loop = asyncio.get_event_loop()
# task = loop.create_task(init())
# loop.run_until_complete(task)
# # ============================================
