use log::{error, info, trace};

use super::super::{NextStep, Rule};
use crate::captures::CaptureData;
use std::collections::VecDeque;

impl Rule {
    pub fn all_rules_for_all_matches(stack: &mut VecDeque<(&Rule, CaptureData)>) -> NextStep {
        trace!(
            "the `all_rules_for_all_matches` method for rule `{}` is running",
            stack.front().unwrap().0.as_ref()
        );
        let mut temp_stack: VecDeque<(&Rule, CaptureData)> = VecDeque::new();
        trace!("temporary stack created");
        while let Some(mut frame) = stack.pop_front() {
            trace!("received frame from stack `{}`", frame.0.as_ref());
            match Self::next_or_data_for_error(frame.0, &mut frame.1) {
                NextStep::Go => {
                    // По каждому тексту в `text_for_capture` мы будем искать совпадения
                    for text in frame.1.text_for_capture.iter() {
                        // Если есть простые подправила, то мы их проверяем
                        if let Some(simple_rules) = &frame
                            .0
                            .content_unchecked()
                            .subrules
                            .as_ref()
                            .unwrap()
                            .simple_rules
                        {
                            // 1 Этап
                            // Получаем правила из `RegexSet`
                            for index in Rule::get_selected_rules(&simple_rules.regex_set, text) {
                                trace!(
                                    "retrieved rules from `RegexSet` : `{}`",
                                    &simple_rules.all_rules[index].as_ref()
                                );
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(
                                    &simple_rules.all_rules[index],
                                    text,
                                );
                                // Сразу узнаем, что будет дальше, если ошибка, то выходим из функции
                                if let NextStep::Error(value) = Self::next_or_data_for_error(
                                    &simple_rules.all_rules[index],
                                    &mut captures,
                                ) {
                                    // ================= (LOG) =================
                                    error!(
                                        "the rule (`{}`, `{:#?}`) did not work for text : `{}`",
                                        &simple_rules.all_rules[index].as_ref(),
                                        &simple_rules.all_rules[index]
                                            .content_unchecked()
                                            .requirement,
                                        text
                                    );
                                    // =========================================
                                    return NextStep::Error(value);
                                }
                                // Загружаем во временный стек если успех
                                temp_stack.push_back((&simple_rules.all_rules[index], captures));
                            }
                            // 2 Этап
                            // Получаем правила, которые не попали в `RegexSet`
                            for rule in simple_rules.all_rules.iter() {
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(rule, text);
                                // Проверяем, что мы не обрабатывали это правило ранее
                                if !temp_stack.iter().any(|&(r, _)| r == rule) {
                                    trace!(
                                        "received rules that are not in `RegexSet` : `{}`",
                                        &rule.as_ref()
                                    );
                                    // Сразу узнаем, что будет дальше, если ошибка, то выходим из функции
                                    if let NextStep::Error(value) =
                                        Self::next_or_data_for_error(rule, &mut captures)
                                    {
                                        // ================= (LOG) =================
                                        error!(
                                            "the rule (`{}`, `{:#?}`) did not work for text : `{}`",
                                            &rule.as_ref(),
                                            &rule.content_unchecked().requirement,
                                            text
                                        );
                                        // =========================================
                                        return NextStep::Error(value);
                                    }
                                    // Загружаем во временный стек, если успех
                                    temp_stack.push_back((rule, captures));
                                }
                            }
                        }
                        // Если есть сложные подправила, то мы их проверяем
                        if let Some(complex_rules) = &frame
                            .0
                            .content_unchecked()
                            .subrules
                            .as_ref()
                            .unwrap()
                            .complex_rules
                        {
                            // 3 Этап
                            // Получаем сложные правила
                            for rule in complex_rules {
                                trace!("received complex rules : `{}`", &rule.as_ref());
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(rule, text);
                                // Сразу узнаем, что будет дальше, если ошибка, то выходим из функции
                                if let NextStep::Error(value) =
                                    Self::next_or_data_for_error(rule, &mut captures)
                                {
                                    // ================= (LOG) =================
                                    error!(
                                        "the rule (`{}`, `{:#?}`) didn't work for the text : `{}`",
                                        &rule.as_ref(),
                                        &rule.content_unchecked().requirement,
                                        text
                                    );
                                    // =========================================
                                    return NextStep::Error(value);
                                }
                                // Загружаем во временный стек если успех
                                temp_stack.push_back((rule, captures));
                            }
                        }
                    }
                }
                NextStep::Finish => (),
                NextStep::Error(value) => {
                    error!(
                        "the rule (`{}`, `{:#?}`) didn't work",
                        &frame.0.as_ref(),
                        &frame.0.content_unchecked().requirement,
                    );
                    return NextStep::Error(value);
                }
            }
        }
        info!("for all matches all rules worked successfully");
        // Финальный этап, мы загружаем всё в`stack` для дальнейшей обработки
        stack.extend(temp_stack.drain(..));
        trace!(
            "stack size: {}\ntemporary stack size: {}",
            stack.len(),
            temp_stack.len()
        );
        NextStep::Finish
    }
}
