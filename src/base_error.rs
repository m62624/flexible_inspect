use super::{BASE_ERROR, MESSAGE_WITH_EXTRA_FROM_CLASS_PY, RULES_FROM_CLASS_PY};
/// Функция для получения базового класса ошибки
pub fn get_base_error() -> String {
    format!(
        "
class PystvalErrorMeta(type):
    def __new__(cls, name, bases, attrs):
        if name == '{}':
            # Добавляем атрибут allow_default_values в класс {}
            attrs['allow_default_values'] = True
        else:
            # Проверяем наличие атрибутов в подклассе
            if '{}' not in attrs or not attrs['{}']:
                raise NotImplementedError(
                    \"Subclasses must provide a non-empty '{}' attribute.\")
            if '{}' not in attrs or not isinstance(attrs['{}'], dict):
                raise NotImplementedError(
                    \"Subclasses must provide a '{}' attribute of type 'dict'.\")

        return super().__new__(cls, name, bases, attrs)


class PystvalError(Exception, metaclass=PystvalErrorMeta):
    pass
",
        BASE_ERROR,
        BASE_ERROR,
        MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
        MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
        MESSAGE_WITH_EXTRA_FROM_CLASS_PY,
        RULES_FROM_CLASS_PY,
        RULES_FROM_CLASS_PY,
        RULES_FROM_CLASS_PY
    )
}
