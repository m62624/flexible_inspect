import pystval
from pystval import Validator


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


class AvatarMissingError(BaseError):
    template = "Avatar is missing or has invalid dimensions (width: {width}, height: {height})"
    rules = ["^hello"]


class UsernameFieldMissingError(BaseError):
    template = "Error: username field is missing or invalid (current name : {name})"
    rules = ["src=<.+\.>"]


class CustomError(BaseError):
    template = "{x}"
    rules = ["aboba"]


# error_true     = True
# x = AvatarMissingError("sdasdad {xl}",)
# try:
#     # throw_error(AvatarMissingError)
# except BaseError as e:
#     print(e.message)
validator1 = Validator([AvatarMissingError, UsernameFieldMissingError])
validator2 = Validator([CustomError])
validator1.validate(b"id=aboba, src=image.pgj")
validator2.validate(b"xxx");
