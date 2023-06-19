import pystval
import asyncio
from pystval import TemplateValidator, MatchRequirement,Rule

def check():
    Rule("Root rule",MatchRequirement.MustBeFound).extend([
        Rule("Subrule-1 of Root",MatchRequirement.MustBeFound),
        Rule("Subrule-2 of Root",MatchRequirement.MustBeFound),
        Rule("Subrule-3 of Root",MatchRequirement.MustBeFound),
        Rule("Subrule-4 of Root",MatchRequirement.MustBeFound).extend([
            Rule("Subrule-1 of Subrule-4",MatchRequirement.MustBeFound),
            Rule("Subrule-2 of Subrule-4",MatchRequirement.MustBeFound),
            Rule("Subrule-3 of Subrule-4",MatchRequirement.MustBeFound),
        ])
        ])


check()