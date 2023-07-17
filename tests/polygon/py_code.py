import asyncio
from pystval import Rule, MatchRequirement, TemplateValidator, PystvalException

text = b"""
---
This is an example of text with different nesting of characters. It has regular letters, numbers, punctuation marks, as well as special characters, emoji and even Unicode characters. Some characters can be nested within each other, such as quotation marks " " or parentheses ( ) [ [123] [123] [1234] ] [ [123456789] ]. Special characters such as the tilde ~ or the dollar sign $ can also be used.
--- 

Validation of text with different nesting of characters may include checking for the presence of paired characters (as in the case of quotes or brackets), correct use of special characters, and compliance with specified rules. For example, if there is an initial quotation mark in the text, there must be a corresponding final quotation mark.

Such validation can be useful, for example, when processing user input, analyzing text, or checking formatting.

I hope this example has helped you visualize how text with different nesting of characters can be used for validation."""


class ErrorCheckText(PystvalException):
    message = "text contains an error"
    rules = [
        Rule(r"---\s?.+\s?---", MatchRequirement.MustBeFound).extend(
            [
                Rule(r"\[[^\[\]]+\]", MatchRequirement.MustBeFound).extend(
                    [
                        Rule(r"\d+", MatchRequirement.MustBeFound),
                        Rule(r"\d{4}", MatchRequirement.MustBeFound)
                    ]).mode_at_least_one_rule_for_all_matches(),
            ]
        )
    ]


simple_text_validator = TemplateValidator([ErrorCheckText])
if simple_text_validator.validate(text) is None:
    print("text is valid")
else:
    for error in simple_text_validator.validate(text):
        try:
            raise error
        except PystvalException as e:
            print(error.report)
