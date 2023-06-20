import pystval
import asyncio
from pystval import TemplateValidator, MatchRequirement,Rule, PystvalError

class CustomError_1(PystvalError):
    message = "my custom error 1"
    rules = [Rule("Root rule",MatchRequirement.MustBeFound)]
    


class CustomError_2(PystvalError):
    message = "my custom error 2"
    rules = [
        Rule("2 Root rule",MatchRequirement.MustBeFound).extend([
        Rule("2 Subrule-1 of Root",MatchRequirement.MustBeFound),
        Rule("^a(?>bc|b)c$(?P<invalid)",MatchRequirement.MustBeFound).extend([
            Rule("2 Subrule-1 of Subrule-2",MatchRequirement.MustBeFound),
            Rule("2 Subrule-2 of Subrule-2",MatchRequirement.MustBeFound),
        ]).extend([
            Rule("2 SubSubrule-1 of Subrule-2",MatchRequirement.MustBeFound),
            Rule("2 SubSubrule-2 of Subrule-2",MatchRequirement.MustBeFound),
        ]),
        Rule("2 Subrule-3 of Root",MatchRequirement.MustBeFound),
        Rule("2 Subrule-4 of Root",MatchRequirement.MustBeFound).extend([
            Rule("2 Subrule-1 of Subrule-4",MatchRequirement.MustBeFound),
            Rule("2 Subrule-2 of Subrule-4",MatchRequirement.MustBeFound),
        ])
        ])]

def main():
    testval = TemplateValidator([CustomError_1,CustomError_2])

main()