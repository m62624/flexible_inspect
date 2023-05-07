from typing import Any, Dict, List


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

    @classmethod
    def classMethod(self, text_raw: bytearray):
        """
        Parameters
        ----------
        `text_raw` : `bytearray`
        Массив байтов
        Raises
        ------
        `TypeError`
        Указан иной тип данных
        `CustomError(your errors)`
        Кастомные ошибки, которые были загружены в конструктор (`flags_errors`)
        """
        pass
    pass
