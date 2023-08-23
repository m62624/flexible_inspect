import unittest
import asyncio
from flexible_inspect_py import Cartridge, MatchRequirement, TemplateValidator, Rule

root_rule = Rule(r"/.+/.+/file.txt", MatchRequirement.MustBeFound)
root_rule.all_r_for_any_m()

cartrdige_0 = Cartridge(0, "Not Found", [
    # root_rule
])

# cartrdige_0 = Cartridge(0, "Not Found", [Rule(
#     r"/.+/.+/file.txt", MatchRequirement.MustBeFound)])
# cartridge_1 = Cartridge(1, "Internal Error {G}", [Rule(
#     r"(?P<G>^(?!.*secret_info).*$)", MatchRequirement.MustNotBeFound)])
# validator_system = TemplateValidator([cartrdige_0, cartridge_1])


# async def init():
#     result = await validator_system.async_validate("10------10")
#     if result is not None:
#         for i in result:
#             print(i.get_message())


# # =============================================
# loop = asyncio.get_event_loop()
# task = loop.create_task(init())
# loop.run_until_complete(task)
# # ============================================
