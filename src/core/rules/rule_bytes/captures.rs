use super::*;
use crate::core::rules::traits::RuleBase;

pub fn find_captures<'a>(rule: &RuleBytes, text_bytes: &'a [u8]) -> CaptureDataBytes<'a> {
    let mut hashmap_for_error: HashMap<String, &[u8]> = HashMap::new();
    let mut text_for_capture = HashSet::new();
    let mut counter_value: usize = 0;
    // флаг для проверки `Counter`
    let flag_check_counter = rule.content_unchecked().general_modifiers.counter.is_some();
    // На первый взгляд мы видим дублирование кода, но каждый `match` работает с разными структурами
    // создаем регулярное выражение
    let re = regex::bytes::Regex::new(&rule.content_unchecked().str_bytes.as_ref()).unwrap();
    // получаем совпадения и повышаем `counter` по необходимости
    re.captures_iter(text_bytes).for_each(|capture| {
        if let Some(value) = capture.get(0) {
            hashmap_for_error
                .entry(DEFAULT_CAPTURE.into())
                .or_insert_with(|| value.as_bytes());
            text_for_capture.insert(value.as_bytes());
            // в одном `regex` может быть несколько групп, но все
            // они нужны для получения главного совпадения, потому
            // повышение идет только в `main capture`
            if flag_check_counter {
                counter_value += 1;
            }
        }
        // получаем совпадения по именам группы, для заполнения `extra` в сообщений ошибки
        re.capture_names().for_each(|name| {
            if let Some(name) = name {
                if let Some(value) = capture.name(name) {
                    hashmap_for_error
                        .entry(name.into())
                        .or_insert_with(|| value.as_bytes());
                }
            }
        })
    });
    CaptureDataBytes {
        text_for_capture,
        hashmap_for_error,
        counter_value,
    }
}
