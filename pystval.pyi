"""`Pystval` is a `Rust` library for `Python`, used to create your own validator."""
import enum
from typing import Type, TypeVar, List,Any

class PystvalErrorMeta(type):
    def __new__(cls, name: str, bases: tuple[type], attrs: dict[str, any]) -> type:
        ...


class PystvalError(Exception, metaclass=PystvalErrorMeta):
    pass

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

    def validate_single_sync(self, text: str):
        """
        Parameters
        ----------
        `text` : `str`
            Text for verification

        Raises
        ------
        `Custom Error (PystvalError)`
            The error that was specified in `flags`
        """
        pass
    pass


