"""`Pystval` is a `Rust' library for `Python', used to create your own validator."""
import enum
from typing import Any, Dict, List


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
        `Custom Error`
            The error that was specified in `flags`
        """
        pass
    pass


class It(enum.Enum):
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
