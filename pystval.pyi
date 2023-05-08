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

    async def validate(self, text: str):
        """
        Parameters
        ----------
        `text` : `str`
            Текст для проверки
        """
        pass
    pass


class It(enum.Enum):
    """
    Перечечисление, где даны варианты действия при положительном результате регулярных выражений
    """
    MustBeFoundHere = 0,
    """
    Должно быть найдено, иначе будет вызвано исключение
    """
    NotToBeFoundHere = 1,
    """
    Не должно быть найдено, иначе будет вызвано исключение
    """
    pass
