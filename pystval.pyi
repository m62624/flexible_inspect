"""`Pystval` - `Rust` библиотека для `Python`, служит для создания собственного валидатора"""
import enum
from typing import Any, Dict, List


class TemplateValidator:
    """
    Класс для создания валидатора
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
        `AttributeError`
            Указан неверный атрибут (отсутствует)
        """
        pass

    async def validate(self, text: str):
        """
        Parameters
        ----------
        `text` : `str`
            Текст для проверки

        Raises
        ------
        `Кастомная ошибка`
            Ошибка, которая была указана в `flags`
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
