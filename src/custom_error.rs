use super::lazy_static;
use super::*;

/// Получаем extra из класса (`MESSAGE_WITH_EXTRA_FROM_CLASS_PY`)
fn extra_from_class(class_template: &types::PyType) -> PyResult<Vec<String>> {
    // Получаем значение атрибута `MESSAGE_WITH_EXTRA_FROM_CLASS_PY`
    let attr_value = class_template
        .getattr(MESSAGE_WITH_EXTRA_FROM_CLASS_PY)?
        .to_string();
    // используем lazy_static, чтобы не создавать каждый раз новый объект
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"\{.+?\}").unwrap();
    }
    // Если есть совпадения, то возвращаем вектор с совпадениями
    // Используем шаблон { имя_группы }, чтобы получить название переменной
    // (да, на данный момент, это еднственный вариант форматирования который можно исполоьзовать, чтобы получить название переменной и заполнить значением)
    if RE.is_match(&attr_value) {
        Ok(RE
            .captures_iter(&attr_value)
            .map(|cap| {
                cap.get(0).map_or(String::new(), |m| {
                    m.as_str().trim_matches('{').trim_matches('}').to_string()
                })
            })
            .collect::<Vec<_>>())
    } else {
        Ok(Vec::new())
    }
}

/// Создаем кастомную ошибк для `python`
pub fn make_error(
    py: Python,
    custom_class_error: &PyObject,
    extra_with_value: &mut Option<HashMap<String, String>>,
) -> PyResult<()> {
    // Создаем словарь для extra
    let extra = types::PyDict::new(py);
    // Получаем extra из класса, это необходимо, если есть `extra` переменные, которых нету в
    // `captures` (`Option<HashMap<String, String>>`). Тем самым мы сможем
    // заполнить пустышкой `___`, чтобы не было ошибки `KeyError`

    /*
    Допустим, в сообщений указано `{name}` и `{age}`. Но в `captures` есть только `{name}`.
    Тогда мы получим ошибку `KeyError`, так как в `extra_with_value` нету ключа `age`.
    Для этого мы получаем `extra` из класса и заполняем `extra_with_value` пустышкой `___`.
     */
    let extra_from_class = extra_from_class(custom_class_error.downcast::<types::PyType>(py)?)?;

    // ================= (LOG) =================
    debug!(
        "Received variables to fill the `{}` message: {:#?}",
        custom_class_error.to_string(),
        extra_from_class
    );
    // =========================================

    // Если есть `extra_with_value`, то мы заполняем `extra` значениями из `extra_with_value`
    if let Some(extra_with_value) = extra_with_value {
        // Заполняем `extra_with_value` пустышкой `___`, при необходимости
        extra_from_class.iter().for_each(|key| {
            extra_with_value.entry(key.into()).or_insert("___".into());
        });
        // Заполняем `extra` значениями из `extra_with_value`
        extra_with_value
            .iter()
            .try_for_each(|(key, value)| extra.set_item(key, value.as_str()))?;
    } else {
        // Заполняем `extra` пустышкой `___`, при необходимости
        extra_from_class.iter().for_each(|key| {
            extra.set_item(key, "___").unwrap();
        });
    }
    // Создаем кастомную ошибку с `extra`
    Err(PyErr::new::<PyException, _>(
        custom_class_error
            .downcast::<PyAny>(py)?
            .call(types::PyTuple::empty(py), Some(extra))?
            .into_py(py),
    ))
}
