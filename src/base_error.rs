use super::{
    BASE_ERROR, EXTRA_FROM_CLASS_PY, MESSAGE_WITH_EXTRA_FROM_CLASS_PY, RULES_FROM_CLASS_PY,
};
/// Функция для получения базового класса ошибки
// Почему именно такая реализация ?
/*
`PyO3` в настоящее время использует функцию `PyErr_NewExceptionWithDoc` из `CPython` для создания типов исключений в модуле `PyErr::new_type`. Однако эта функция создает только классы исключений `Python`, которые имеют ограниченные возможности для добавления или изменения атрибутов экземпляра. Аргументы функции `PyErr_NewExceptionWithDoc` предназначены только для определения переменных и методов класса `dict`, и не могут быть использованы для добавления атрибутов экземпляра.
 */
pub fn get_base_error() -> String {
    format!("
class {BASE_ERROR}Meta(type):
        def __new__(cls, name, bases, attrs):
            if name == '{BASE_ERROR}':
                attrs['allow_default_values'] = True
            else:
                if '{MESSAGE_WITH_EXTRA_FROM_CLASS_PY}' not in attrs or not attrs['{MESSAGE_WITH_EXTRA_FROM_CLASS_PY}']:
                    raise NotImplementedError(
                        \"Subclasses must provide a non-empty '{MESSAGE_WITH_EXTRA_FROM_CLASS_PY}' attribute.\")
                if '{RULES_FROM_CLASS_PY}' not in attrs or not isinstance(attrs['{RULES_FROM_CLASS_PY}'], dict):
                    raise NotImplementedError(
                        \"Subclasses must provide a '{RULES_FROM_CLASS_PY}' attribute of type 'dict'.\")
            return super().__new__(cls, name, bases, attrs)
    
    
class {BASE_ERROR}(Exception,metaclass={BASE_ERROR}Meta):
        {MESSAGE_WITH_EXTRA_FROM_CLASS_PY} = \"\"
    
        def __init__(self, {MESSAGE_WITH_EXTRA_FROM_CLASS_PY}: str = None, {RULES_FROM_CLASS_PY}: dict[str, enumerate] = None, **{EXTRA_FROM_CLASS_PY}):
            self.__{EXTRA_FROM_CLASS_PY} = {EXTRA_FROM_CLASS_PY}
            self.__{RULES_FROM_CLASS_PY} = {RULES_FROM_CLASS_PY}
            if {MESSAGE_WITH_EXTRA_FROM_CLASS_PY} is None:
                self.__{MESSAGE_WITH_EXTRA_FROM_CLASS_PY} = self.{MESSAGE_WITH_EXTRA_FROM_CLASS_PY}.format(**{EXTRA_FROM_CLASS_PY})
            else:
                self.__{MESSAGE_WITH_EXTRA_FROM_CLASS_PY} = {MESSAGE_WITH_EXTRA_FROM_CLASS_PY}.format(**{EXTRA_FROM_CLASS_PY})
    
    
        @property
        def report(self):
            return self.__{MESSAGE_WITH_EXTRA_FROM_CLASS_PY}
    
        @property
        def {EXTRA_FROM_CLASS_PY}(self):
            return self.__{EXTRA_FROM_CLASS_PY}
    
        @property
        def {RULES_FROM_CLASS_PY}(self):
            return self.__{RULES_FROM_CLASS_PY}
"
    )
}
