import pystval
import asyncio
from pystval import TemplateValidator, It, throw_base_error,BaseErrorWrapper

class check_error(BaseErrorWrapper):
    pass




class FindSomething(BaseErrorWrapper):
    template = "Gotcha : {OS}"
    rules = {
        r"""(?P<OS>Linux)""": It.MustBeFoundHere,
    }

# # ============================================


async def init():
    # text = await get_bytes("-- link --")
    with open('Makefile', 'rb') as file:
        text = file.read()
    validator_sample = TemplateValidator(
        flags=[FindSomething]),
    try:
        await validator_sample.validate(text)
    except BaseErrorWrapper as e:
        print(f"ERROR VALIDATE: '{e.message}'")


# # # =============================================
# loop = asyncio.get_event_loop()
# task = loop.create_task(init())
# loop.run_until_complete(task)
# # ============================================
