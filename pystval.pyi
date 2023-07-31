import enum
from typing import Optional, Type, List, Any


# ========================================================
class MatchRequirement(enum.Enum):
    MustBeFound: int
    MustNotBeFound: int
# ========================================================


# ========================================================
class Rule:
    def __init__(self, pattern: str, requirement: MatchRequirement) -> None:
        ...

    def extend(self, nested_rules: List[Rule]) -> Rule:
        """
    Extend the rule with nested rules.

    Parameters
    ----------
    `nested_rules` : `List[Rule]`
        The nested rules to be added

    Raises
    ------
    `TypeError`
        If `nested_rules` is not a list of `Rule` objects

        """
    ...

    def counter_is_equal(self, count: int) -> Rule:
        """
    Adding a match counter, where the condition is: there must be exactly `count` matches
        """
    ...

    def counter_more_than(self, count: int) -> Rule:
        """
    Adding a match counter, where the condition is: there must be greater than or equal to `count` matches
        """
    ...

    def counter_less_than(self, count: int) -> Rule:
        """
    Adding a match counter, where the condition is: there must be less than or equal to `count` matches
        """
    ...

    def mode_all_rules_for_at_least_one_match(self) -> Rule:
        """
    All subrules should work successfully for at least one match (text)
        """
    ...

    def mode_at_least_one_rule_for_all_matches(self) -> Rule:
        """
    At least one rule should work successfully for all matches
        """
    ...

    def mode_at_least_one_rule_for_at_least_one_match(self) -> Rule:
        """
    At least one rule should work successfully for at least one match
        """
    ...
# ========================================================


# ========================================================
class RuleBytes:
    def __init__(self, pattern: str, requirement: MatchRequirement) -> None:
        ...

    def extend(self, nested_rules: List[RuleBytes]) -> RuleBytes:
        """
    Extend the rule with nested rules.

    Parameters
    ----------
    `nested_rules` : `List[Rule]`
        The nested rules to be added

    Raises
    ------
    `TypeError`
        If `nested_rules` is not a list of `Rule` objects

        """
    ...

    def counter_is_equal(self, count: int) -> RuleBytes:
        """
    Adding a match counter, where the condition is: there must be exactly `count` matches
        """
    ...

    def counter_more_than(self, count: int) -> RuleBytes:
        """
    Adding a match counter, where the condition is: there must be greater than or equal to `count` matches
        """
    ...

    def counter_less_than(self, count: int) -> RuleBytes:
        """
    Adding a match counter, where the condition is: there must be less than or equal to `count` matches
        """
    ...

    def mode_all_rules_for_at_least_one_match(self) -> RuleBytes:
        """
    All subrules should work successfully for at least one match (text)
        """
    ...

    def mode_at_least_one_rule_for_all_matches(self) -> RuleBytes:
        """
    At least one rule should work successfully for all matches
        """
    ...

    def mode_at_least_one_rule_for_at_least_one_match(self) -> RuleBytes:
        """
    At least one rule should work successfully for at least one match
        """
    ...
# ========================================================


# ========================================================
class Cartridge:
    def __init__(self, id: int, message: str, root_rules: List[Rule]) -> None:
        ...

    def mode_all_rules_for_at_least_one_match(self) -> Cartridge:
        ...

    def mode_at_least_one_rule_for_all_matches(self) -> Cartridge:
        ...

    def mode_at_least_one_rule_for_at_least_one_match(self) -> Cartridge:
        ...
# ========================================================


# ========================================================
class CartridgeBytes:
    def __init__(self, id: int, message: str, root_rules: List[RuleBytes]) -> None:
        ...

    def mode_all_rules_for_at_least_one_match(self) -> CartridgeBytes:
        ...

    def mode_at_least_one_rule_for_all_matches(self) -> CartridgeBytes:
        ...

    def mode_at_least_one_rule_for_at_least_one_match(self) -> CartridgeBytes:
        ...
# ========================================================


# ========================================================
class TemplateValidator:
    def __init__(self, rules: List[Cartridge]) -> None:
        ...

    async def async_validate(self, text: str) -> None | List[PystvalException]:
        ...

    def validate(self, text: str) -> None | List[PystvalException]:
        ...
# ========================================================


# ========================================================
class TemplateValidatorBytes:
    def __init__(self, rules: List[CartridgeBytes]) -> None:
        ...

    async def async_validate(self, text: bytes) -> None | List[PystvalException]:
        ...

    def validate(self, text: bytes) -> None | List[PystvalException]:
        ...
# ==========================================================


# ========================================================
class PystvalException(Exception):

    def get_code(self) -> int:
        ...
# ========================================================
