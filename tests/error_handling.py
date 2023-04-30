import pystval
from pystval import Validator, throw_error

# Шаблон ошибки


class BaseError(Exception):
    template = ""

    def __init__(self, message: str = None, **extra):
        self._extra = extra
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

# Последующие две ошибки является тестовыми, необходимы для проверки
# функций поглощение ошибок на основе BaseError


class AvatarMissingError(BaseError):
    template = "Avatar is missing or has invalid dimensions (width: {width}, height: {height})"


class UsernameFieldMissingError(BaseError):
    template = "Error: username field is missing or invalid (current name : {name})"


class QALS:
    pass


# Dict, является набором флажков и ошибок для `фабрики pystval`
element_keys = {
    'avatar': AvatarMissingError,
    'username_field': UsernameFieldMissingError,
    "aboba error": QALS,
}

# e = AvatarMissingError(width=100, height=200)
# message = e.message
# print(message)
error_true = True
# x = Validator(element_keys, BaseError)
print(throw_error(AvatarMissingError))
