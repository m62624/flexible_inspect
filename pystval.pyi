"""`Pystval` is a `Rust` library for `Python`, used to create your own validator."""
import enum
from typing import Any, Dict, List


class PystvalError(Exception):
    """
    A class for creating your own exceptions
    """
    message: str
    extra: Dict[str, Any]
    rules: Dict[str, Any]

    def __init__(self, message: str, extra: Dict[str, Any] = None, rules: Dict[str, Any] = None) -> None:
        """
        Parameters
        ----------
        `message` : `str`
            Message for exception
        `extra` : `Dict[str, Any]`
            Additional data
        `rules` : `Dict[str, Any]`
            Rules for validation
        """
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
        pass

    async def validate(self, text: str):
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


class MatchRequirement(enum.Enum):
    """
    A `enumeration` that gives options on what to do when you find a regex match 
    """
    MustBeFoundHere = 0,
    """
    It must be found, otherwise an exception will be thrown
    """
    NotToBeFoundHere = 1,
    """
    It is not to Be found here, otherwise an exception will be raised
    """
    pass
