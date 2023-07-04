import pystval
import asyncio
from pystval import Rule,MatchRequirement,TemplateValidator,PystvalException

class ErrorAboba(PystvalException):
    message = "Not found aboba"
    rules = [
        Rule("aboba", MatchRequirement.MustBeFound)
    ]
    pass

class ErrorNumber(PystvalException):
    message = "found number {n}"
    rules = [
        Rule("(?P<n>\d+)", MatchRequirement.MustNotBefound)
    ]
    pass

def init():
    validator = TemplateValidator([ErrorAboba, ErrorNumber])
    list_error = validator.validate(b"aboba 123")
    print(list_error)
    if list_error is not None:
        for error in list_error:
            try:
                raise error
            except PystvalException as e:
                print("work ?")
                print(e)

init()