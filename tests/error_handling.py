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
    rules = ["regex template"]


class UsernameFieldMissingError(BaseError):
    template = "Error: username field is missing or invalid (current name : {name})"
    rules = ["regex template"]


# error_true = True
validator = Validator([AvatarMissingError, UsernameFieldMissingError])
