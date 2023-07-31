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
    modifier to set the match counter, condition counter == match
        """
    ...

    def counter_more_than(self, count: int) -> Rule:
        """
    modifier to set the match counter, condition counter >= match
        """
    ...

    def counter_less_than(self, count: int) -> Rule:
        """
    modifier to set the match counter, condition counter <= match
        """
    ...

    def mode_all_rules_for_at_least_one_match(self) -> Rule:
        """
    modifier to change the rule matching mode,
    `all rules` must pass the test for at least `one match`
        """
    ...

    def mode_at_least_one_rule_for_all_matches(self) -> Rule:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for `all matches`
        """
    ...

    def mode_at_least_one_rule_for_at_least_one_match(self) -> Rule:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for at least `one match`
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
    modifier to set the match counter, condition counter == match
        """
    ...

    def counter_more_than(self, count: int) -> RuleBytes:
        """
    modifier to set the match counter, condition counter >= match
        """
    ...

    def counter_less_than(self, count: int) -> RuleBytes:
        """
    modifier to set the match counter, condition counter <= match
        """
    ...

    def mode_all_rules_for_at_least_one_match(self) -> RuleBytes:
        """
    modifier to change the rule matching mode,
    `all rules` must pass the test for at least `one match`
        """
    ...

    def mode_at_least_one_rule_for_all_matches(self) -> RuleBytes:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for `all matches`
        """
    ...

    def mode_at_least_one_rule_for_at_least_one_match(self) -> RuleBytes:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for at least `one match`
        """
    ...
# ========================================================


# ========================================================
class Cartridge:
    def __init__(self, id: int, message: str, root_rules: List[Rule]) -> None:
        ...

    def mode_all_rules_for_at_least_one_match(self) -> Cartridge:
        """
    modifier to change the rule matching mode,
    `all rules` must pass the test for at least `one match`
        """
        ...

    def mode_at_least_one_rule_for_all_matches(self) -> Cartridge:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for `all matches`
        """
        ...

    def mode_at_least_one_rule_for_at_least_one_match(self) -> Cartridge:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for at least `one match`
        """
        ...
# ========================================================


# ========================================================
class CartridgeBytes:
    def __init__(self, id: int, message: str, root_rules: List[RuleBytes]) -> None:
        ...

    def mode_all_rules_for_at_least_one_match(self) -> CartridgeBytes:
        """
    modifier to change the rule matching mode,
    `all rules` must pass the test for at least `one match`
        """
        ...

    def mode_at_least_one_rule_for_all_matches(self) -> CartridgeBytes:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for `all matches`
        """
        ...

    def mode_at_least_one_rule_for_at_least_one_match(self) -> CartridgeBytes:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for at least `one match`
        """
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
