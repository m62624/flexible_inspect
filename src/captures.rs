use super::rule::{RegexRaw, Rule};
use std::collections::{HashMap, HashSet};

/// Структура для хранения типа захвата, с дублированием данных или без
#[derive(Debug)]
pub enum CaptureType<'s> {
    Single(HashSet<&'s str>),
    Multiple(Vec<&'s str>),
}

/// Структура для хранения совпадений
#[derive(Debug)]
pub struct CaptureData<'s> {
    /// Хранит совпадения по именам, для заполнения `extra` в сообщений ошибки
    pub hashmap_for_error: HashMap<String, String>,
    // Хранит совпадения по тексту, для проверок подправил
    pub text_for_capture: CaptureType<'s>,
    // Хранит количество совпадений, для проверки `Counter`
    pub counter_value: usize,
}

impl<'s> CaptureData<'s> {
    /// Метод для получения совпадений
    pub fn find_captures(rule: &Rule, text: &'s str) -> Self {
        let mut hashmap_for_error = HashMap::new();
        let mut text_for_capture = HashSet::new();
        let mut text_for_capture_duplicate = Vec::new();
        let mut counter: usize = 0;
        // флаг для проверки `Counter`
        let flag_check_counter = rule.content_unchecked().counter.is_some();
        // На первый взгляд мы видим дублирование кода, но каждый `match` работает с разными структурами

        match &rule.content_unchecked().str_with_type {
            RegexRaw::DefaultR(pattern) => {
                // создаем регулярное выражение
                let re = regex::Regex::new(pattern).unwrap();
                // получаем совпадения и повышаем `counter` по необходимости
                re.captures_iter(text).for_each(|capture| {
                    if let Some(value) = capture.get(0) {
                        hashmap_for_error
                            .entry("main_capture".into())
                            .or_insert_with(|| value.as_str().into());
                        if rule.content_unchecked().duplicate_matches {
                            text_for_capture_duplicate.push(value.as_str());
                        } else {
                            text_for_capture.insert(value.as_str());
                        }
                        // в одном `regex` может быть несколько групп, но все
                        // они нужны для получения главного совпадения, потому
                        // повышение идет только в `main capture`
                        if flag_check_counter {
                            counter += 1;
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
            RegexRaw::FancyR(pattern) => {
                // создаем регулярное выражение
                let re = fancy_regex::Regex::new(pattern).unwrap();
                // получаем совпадения и повышаем `counter` по необходимости
                re.captures_iter(text).for_each(|capture| {
                    if let Ok(capture) = capture {
                        if let Some(value) = capture.get(0) {
                            hashmap_for_error
                                .entry("main_capture".into())
                                .or_insert_with(|| value.as_str().into());
                            if rule.content_unchecked().duplicate_matches {
                                text_for_capture_duplicate.push(value.as_str());
                            } else {
                                text_for_capture.insert(value.as_str());
                            }
                            // в одном `regex` может быть несколько групп, но все
                            // они нужны для получения главного совпадения, потому
                            // повышение идет только в `main capture`
                            if flag_check_counter {
                                counter += 1;
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
        // возвращаем структуру
        Self {
            hashmap_for_error,
            text_for_capture: if rule.content_unchecked().duplicate_matches {
                CaptureType::Multiple(text_for_capture_duplicate)
            } else {
                CaptureType::Single(text_for_capture)
            },
            counter_value: counter,
        }
    }
    /// Проверка присутствия совпадений
    pub fn is_some(&self) -> bool {
        match &self.text_for_capture {
            CaptureType::Single(s) => !s.is_empty(),
            CaptureType::Multiple(v) => !v.is_empty(),
        }
    }
}
