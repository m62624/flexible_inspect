import enum
from typing import Optional, Type, List, Any


class PystvalErrorMeta(type):
    def __new__(cls, name: str, bases: tuple[type], attrs: dict[str, Any]) -> type:
        ...


class PystvalException(Exception, metaclass=PystvalErrorMeta):
    """
    Custom exception class for Pystval library.

    Attributes
    ----------
    `report` : `str`
        The report message associated with the exception.
    """
    @property
    def report(self) -> str:
        """
        Get the report message associated with the exception.

        Returns
        -------
        `str`
            The report message.

        """
        ...

    pass


class MatchRequirement(enum.Enum):
    """
    An enumeration that gives options on what to do when you find a regex match 
    """
    MustBeFound = 0
    """
    It must be found, otherwise an exception will be thrown
    """
    MustNotBeFound = 1
    """
    It is not to be found here, otherwise an exception will be raised
    """


class Rule:
    """
    A class for storing a rule with various modifiers and subrules
    """

    def __init__(self, inner: str, requirements: MatchRequirement) -> None:
        """
        Parameters
        ----------
        `inner` : `str`
            The inner value for the rule
        `requirements` : `MatchRequirement`
            The match requirement for the rule

        Raises
        ------
        `TypeError`
            If the inner parameter is not a string

        """
        ...

    @staticmethod
    def auto_generate(requirements: MatchRequirement, texts: List[str]) -> Rule:
        """
        Automatically generate a rule based on the inner value
        > Recommendation : use for pattern prototyping
        """
        ...

    def extend(self, nested_rules: List['Rule']) -> Rule:
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

    def mode_all_rules_for_all_matches(self) -> Rule:
        """
        All subrules should work successfully for all matches (text)
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

    def duplicate_matches(self) -> Rule:
        """
        Allow duplicate matches
        """
        ...


class TemplateValidator:
    """
    A class for creating a validator
    """

    def __init__(self, flags: List[Type[Any]]) -> None:
        """
        Parameters
        ----------
        `flags` : `List[Type[Any]]`
            List of classes

        Raises
        ------
        `TypeError`
            If a different data type is specified
        `AttributeError`
            If an incorrect attribute is specified or missing
        """
        ...

    async def async_validate(self, text: bytes) -> None:
        """
        Parameters
        ----------
        `text` : `bytes`
            Text for verification

        Raises
        ------
        `CustomError(PystvalException)`
           Custom error (`PystvalException`), which were specified in `flags`.
        """
        ...

    def validate(self, text: bytes) -> Optional[List[PystvalException]]:
        """
        Parameters
        ----------
        `text` : `bytes`
            Text for verification

        Raises
        ------
        `List[CustomError(PystvalException)]`
            List of custom errors (`PystvalException`) that were specified in `flags`
        """
        ...
