"""
The Data validator is a universal tool for checking the correctness of data in string and byte formats. It allows you to determine whether the data conforms to certain rules and provides the ability to find errors and inconsistencies.\\
The project aims to create a versatile and flexible tool for validating data in different formats, ensuring accuracy, reliability and usability.
"""

import enum
from collections.abc import Iterator
from typing import Optional, Type, List, Any, TypeVar, Union


# ========================================================
class MatchRequirement(enum.Enum):
    """
    The structure that defines what action is required when finding regular expression matches.
    """
    MustBeFound = 0
    """
    The match must be found
    """
    MustNotBeFound = 1
    """
    the match must not be found
    """
# ========================================================


# ========================================================
class Rule:
    """
    The structure for checking strings with regular expressions
    """

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

    def all_r_for_any_m(self) -> Rule:
        """
    modifier to change the rule matching mode,
    `all rules` must pass the test for at least `one match`
        """
    ...

    def any_r_for_all_m(self) -> Rule:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for `all matches`
        """
    ...

    def any_r_for_any_m(self) -> Rule:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for at least `one match`
        """
    ...
# ========================================================


# ========================================================
class RuleBytes:
    """
    The structure for checking bytes with regular expressions
    """

    def __init__(self, pattern: str, requirement: MatchRequirement) -> None:
        ...

    def extend(self, nested_rules: List[RuleBytes]) -> RuleBytes:
        """
    Extend the rule with nested rules.

    Parameters
    ----------
    `nested_rules` : `List[RuleBytes]`
        The nested rules to be added

    Raises
    ------
    `TypeError`
        If `nested_rules` is not a list of `RuleBytes` objects

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

    def all_r_for_any_m(self) -> RuleBytes:
        """
    modifier to change the rule matching mode,
    `all rules` must pass the test for at least `one match`
        """
    ...

    def any_r_for_all_m(self) -> RuleBytes:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for `all matches`
        """
    ...

    def any_r_for_any_m(self) -> RuleBytes:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for at least `one match`
        """
    ...
# ========================================================


# ========================================================
class Cartridge:
    """
 The cartridge is the container of the rules.
 # Notes
 * Use a container for one object if possible. Imagine that one container is one specific error `NotFound`, `InvalidHeader`, `WrongCase`.
    """

    def __init__(self, id: int, message: str, root_rules: List[Rule]) -> None:
        """
        Constructor for `Cartridge`, each cartridge can only hold one type at a time, `Rule` or `RuleBytes`
        """
        ...

    def any_r_for_any_m(self) -> Cartridge:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for at least `one match`
        """
        ...

# ========================================================


# ========================================================
class CartridgeBytes:
    """
 The cartridge is the container of the rules.
 # Notes
 * Use a container for one object if possible. Imagine that one container is one specific error `NotFound`, `InvalidHeader`, `WrongCase`.
    """

    def __init__(self, id: int, message: str, root_rules: List[RuleBytes]) -> None:
        ...

    def any_r_for_any_m(self) -> CartridgeBytes:
        """
    modifier to change the rule matching mode,
    at least `one rule` must pass the test for at least `one match`
        """
        ...
# ========================================================


# ========================================================
class TemplateValidator:
    """
    The structure for creating unique validators, load different `cartridges` to validate data.
    """

    def __init__(self, rules: List[Cartridge]) -> None:
        ...

    async def async_validate(self, text: str) -> Optional[Iterator[ValidationError]]:
        """
        Parameters
        ----------
        `data` : `str`
            Data for verification

        Raises
        ------
        `ValidationErrorIterator`
            This iterator stores an `error code` and a `message`
        """
        ...

    def validate(self, text: str) -> Optional[Iterator[ValidationError]]:
        """
        Parameters
        ----------
        `data` : `str`
            Data for verification

        Raises
        ------
        `ValidationErrorIterator`
           This error stores an `error code` and a `message`
        """
        ...
# ========================================================


# ========================================================
class TemplateValidatorBytes:
    """
    The structure for creating unique validators, load different `cartridges` to validate data.
    """

    def __init__(self, rules: List[CartridgeBytes]) -> None:
        ...

    async def async_validate(self, text: bytes) -> Optional[Iterator[ValidationError]]:
        """
        Parameters
        ----------
        `data` : `bytes`
            Data for verification

        Raises
        ------
        `ValidationErrorIterator`
           This error stores an `error code` and a `message`
        """
        ...

    def validate(self, text: bytes) -> Optional[Iterator[ValidationError]]:
        """
        Parameters
        ----------
        `data` : `bytes`
            Data for verification

        Raises
        ------
        `ValidationErrorIterator`
           This error stores an `error code` and a `message`
        """
        ...
        ...
# ==========================================================


class ValidationError:
    pass

    def get_code(self) -> int:
        """
        Returns
        -------
        `int`
            Error code
        """
        ...

    def get_message(self) -> str:
        """
        Returns
        -------
        `str`
            Error message with data
        """
        ...
