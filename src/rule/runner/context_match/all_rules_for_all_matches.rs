use super::*;

impl Rule {
    /*
    Используется `for` вместо `iterator`, так как возращаем `NextStep`, при
    использований цикла (`for`), мы можем сделать ранний выход из функции, если
    возникла ошибка
     */

    /// Проверяем, что на каждое совпадение (текст), сработают все правила
    pub fn all_rules_for_all_matches(stack: &mut VecDeque<(&Rule, CaptureData)>) -> NextStep {
        // Создаем временный стек, в который будем складывать все правила, которые нужно обработать
        let mut temp_stack: VecDeque<(&Rule, CaptureData)> = VecDeque::new();
        // Начнем проход по `stack`, `stack_temp` будет расширять `stack`
        while let Some(mut frame) = stack.pop_front() {
            // ================= (LOG) =================
            trace!(
                "Started rule (`{}`, `{:#?}`) from the stack\nFull details of the rule (after modifications): {:#?}",
                frame.0.as_ref(),
                frame.0.content_unchecked().requirement,
                frame.0
            );
            // =========================================
            // Проверяем, что мы можем продолжить выполнение правила, если нет, то либо пропуск, либо ошибка
            match Self::next_or_data_for_error(frame.0, &mut frame.1) {
                NextStep::Go => {
                    // По каждому тексту в `text_for_capture` мы будем искать совпадения
                    for text in frame.1.text_for_capture.iter() {
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
                                        "The rule (`{}`, `{:#?}`) did not work for text : `{}`",
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
                                if !stack.iter().any(|&(r, _)| r == rule) {
                                    // Сразу узнаем, что будет дальше, если ошибка, то выходим из функции
                                    if let NextStep::Error(value) =
                                        Self::next_or_data_for_error(rule, &mut captures)
                                    {
                                        // ================= (LOG) =================
                                        error!(
                                            "The rule (`{}`, `{:#?}`) did not work for text : `{}`",
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
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(rule, text);
                                // Сразу узнаем, что будет дальше, если ошибка, то выходим из функции
                                if let NextStep::Error(value) =
                                    Self::next_or_data_for_error(rule, &mut captures)
                                {
                                    // ================= (LOG) =================
                                    error!(
                                        "The rule (`{}`, `{:#?}`) did not work for text : `{}`",
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
                    // Финальный этап, мы загружаем всё в`stack` для дальнейшей обработки
                    stack.extend(temp_stack.drain(..));
                }
                NextStep::Finish => (),
                NextStep::Error(value) => return NextStep::Error(value),
            }
        }
        NextStep::Finish
    }
}
