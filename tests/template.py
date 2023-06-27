import pystval
import asyncio
from pystval import MatchRequirement,Rule

rule = Rule("\d",MatchRequirement.MustBeFound)