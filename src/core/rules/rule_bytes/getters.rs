use std::collections::HashSet;

use super::*;
use crate::core::rules::traits::{RuleBase, RuleBytesExtendBase};

impl RuleBase for RuleBytes {
    type TakeRuleType = TakeRuleBytesForExtend;
    /// Use for direct access to the structure body
    fn content_unchecked(&self) -> &TakeRuleBytesForExtend {
        self.0.as_ref().expect(ERR_OPTION)
    }

    /// Use for direct access and modification to the body of the structure
    fn content_mut_unchecked(&mut self) -> &mut TakeRuleBytesForExtend {
        self.0.as_mut().expect(ERR_OPTION)
    }
}

impl<'a> RuleBytesExtendBase<'a> for RuleBytes {
    /// Get selected rules from `RegexSet`
    fn get_selected_rules(regex_set: &regex::bytes::RegexSet, text_bytes: &[u8]) -> Vec<usize> {
        regex_set.matches(text_bytes).iter().collect()
    }

    fn find_captures(rule: &RuleBytes, text_bytes: &'a [u8]) -> CaptureDataBytes<'a> {
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
}
