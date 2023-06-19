"""`Pystval` is a `Rust` library for `Python`, used to create your own validator."""
import enum
from typing import Any, Dict, List


class PystvalError(Exception):
    """
    A class for creating your own exceptions
    Parameters
    ----------
    `message` : `str`
        Message for exception
    `rules` : `List[Any]`
        Rules for validation
    """

    message: str
    extra: Dict[str, Any]
    rules: List[Any]

    def __init__(self, message: str, extra: Dict[str, Any] = None, rules: List[Any] = None) -> None:
        """
        Parameters
        ----------
        `message` : `str`
            Message for exception
        `rules` : `List[Any]`
            Rules for validation
        """
        ...

    @property
    def report(self) -> str:
        """
        Get the formatted error message for the exception.
        """
        ...

class MatchRequirement(enum.Enum):
    """
    A `enumeration` that gives options on what to do when you find a regex match 
    """
    MustBeFound = 0,
    """
    It must be found, otherwise an exception will be thrown
    """
    MustNotBeFound = 1,
    """
    It is not to Be found here, otherwise an exception will be raised
    """

class Rule:
    def __init__(self, inner: str, requirements: MatchRequirement) -> None:
        ...
    
    def extend(self, nested_rules: List[Rule]) -> None:
        ...


class TemplateValidator:
    """
    A class for creating a validator
    """

    def __init__(self, flags: List[Any]):
        """
        Parameters
        ----------
        `flags` : `List[Any]`
            List of classes

        Raises
        ------
        `TypeError`
            Specifies a different data type
        `AttributeError`
            Incorrect attribute specified (missing)
        """

    # async def async_validate(self, text: str):
    #     """
    #     Parameters
    #     ----------
    #     `text` : `str`
    #         Text for verification

    #     Raises
    #     ------
    #     `Custom Error (PystvalError)`
    #         The error that was specified in `flags`
    #     """
    #     pass
    # pass

    # def validate(self, text: str):
    #     """
    #     Parameters
    #     ----------
    #     `text` : `str`
    #         Text for verification

    #     Raises
    #     ------
    #     `Custom Error (PystvalError)`
    #         The error that was specified in `flags`
    #     """
    #     pass
    # pass


