from flexible_inspect_py import Cartridge, CartridgeBytes, MatchRequirement, TemplateValidator, TemplateValidatorBytes, Rule, RuleBytes


def my_function(x, y):
    print("Error code: ", "Message: ", x, y)


error_404 = Cartridge(404,
                      "Not Found",
                      [Rule(r"/.+/.+/file.txt", MatchRequirement.MustBeFound)]
                      )

error_panic_system = Cartridge(500,
                               "Internal Error {G}",
                               [Rule(r"(?P<G>^(?!.*secret_info).*$)",
                                     MatchRequirement.MustNotBeFound)]
                               )

valdiator_system = TemplateValidator([error_404, error_panic_system])
iterator_rust = valdiator_system.validate("10------10")

for error in iterator_rust:
    print(error.get_code(), error.get_message())
# print(iterator_rust.__next__().get_code())
