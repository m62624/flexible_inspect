from typing import Any, Dict, List
import enum


class Validator:
    """
    Класс для создания собственного валидатора
    """

    def __init__(self, flags_errors: List[Any]):
        """
        Parameters
        ----------
        `flags_errors` : `List[Any]`
            Список классов

        Raises
        ------
        `TypeError`
            Указан иной тип данных
        """
        pass
    pass


class IfFound(enum.Enum):
    DoNothing = 0,
    RaiseError = 1,
    """
    Перечечисление, где даны варианты действия при положительном результате регулярных выражений
    """
    pass
