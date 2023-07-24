use super::*;
use crate::core::rules::traits::RuleBase;

pub fn find_captures<'a>(rule: &Rule, text: &'a str) -> CaptureData<'a> {
    let mut hashmap_for_error = HashMap::new();
    let mut text_for_capture: HashSet<CaptureType> = HashSet::new();
    let mut counter_value: usize = 0;
    // флаг для проверки `Counter`
    let flag_check_counter = rule.content_unchecked().general_modifiers.counter.is_some();
    // На первый взгляд мы видим дублирование кода, но каждый `match` работает с разными структурами

    match &rule.content_unchecked().str_with_type {
        RegexRaw::DefaultRegex(pattern) => {
            // создаем регулярное выражение
            let re = regex::Regex::new(pattern).unwrap();
            // получаем совпадения и повышаем `counter` по необходимости
            re.captures_iter(text).for_each(|capture| {
                if let Some(value) = capture.get(0) {
                    hashmap_for_error
                        .entry(DEFAULT_CAPTURE.into())
                        .or_insert_with(|| value.as_str().into());
                    text_for_capture.insert(CaptureType::Str(value.as_str()));
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
                                .or_insert_with(|| value.as_str().into());
                        }
                    }
                })
            });
        }
        RegexRaw::FancyRegex(pattern) => {
            // создаем регулярное выражение
            let re = fancy_regex::Regex::new(pattern).unwrap();
            // получаем совпадения и повышаем `counter` по необходимости
            re.captures_iter(text).for_each(|capture| {
                if let Ok(capture) = capture {
                    if let Some(value) = capture.get(0) {
                        hashmap_for_error
                            .entry(DEFAULT_CAPTURE.into())
                            .or_insert_with(|| value.as_str().into());
                        text_for_capture.insert(CaptureType::Str(value.as_str()));
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
                                    .or_insert_with(|| value.as_str().into());
                            }
                        }
                    })
                }
            });
        }
    }
    CaptureData {
        text_for_capture,
        hashmap_for_error,
        counter_value,
    }
}

