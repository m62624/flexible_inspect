import pystval
from pystval import TemplateValidator, TemplateValidatorBytes, Rule, RuleBytes, Cartridge, CartridgeBytes,MatchRequirement

rule1 = Rule(r"\d",MatchRequirement.MustBeFound)
rule2 = RuleBytes(r"\d",MatchRequirement.MustBeFound)
pre_final =  Cartridge(1,"x",[rule1])
final = TemplateValidator([pre_final])