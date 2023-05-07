from typing import Any, Dict, List
import enum


class TemplateValidator:
    """
    Класс для создания собственного валидатора
    """

    def __init__(self, flags: List[Any]):
        """
        Parameters
        ----------
        `flags` : `List[Any]`
            Список классов

        Raises
        ------
        `TypeError`
            Указан иной тип данных
        """
        pass
    pass


class IfFound(enum.Enum):
    """
    Перечечисление, где даны варианты действия при положительном результате регулярных выражений
    """
    AllRight = 0,
    RaiseError = 1,
    pass
