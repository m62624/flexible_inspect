import pystval
import asyncio
from pystval import TemplateValidator, It, PystvalError

print(PystvalError.__base__)


class FindSomething(PystvalError):
    message = "Gotcha : {OS}"
    rules = {
        r"""(?P<OS>Linux)""": It.MustBeFoundHere,
    }

# # ============================================


# async def init():
#     # text = await get_bytes("-- link --")
#     with open('Makefile', 'rb') as file:
#         text = file.read()
#     validator_sample = TemplateValidator(flags=[FindSomething])
#     try:
#         await validator_sample.validate(text)
#     except PystvalError as e:
#         print(f"ERROR VALIDATE: '{e.message}'")


# # # # =============================================
# loop = asyncio.get_event_loop()
# task = loop.create_task(init())
# loop.run_until_complete(task)
# # ============================================
