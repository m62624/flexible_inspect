use super::super::*;

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
        trace!("temporary stack created");
        // Начнем проход по `stack`, `stack_temp` будет расширять `stack`
        // Создаем map для отслеживания успеха правил
        while let Some(mut frame) = stack.pop_front() {
            // Проверяем, нужно ли идти дальше
            match Self::next_or_data_for_error(frame.0, &mut frame.1) {
                NextStep::Go => {
                    let texts: Vec<_> = match &frame.1.text_for_capture {
                        captures::CaptureType::Single(v) => v.iter().collect(),
                        captures::CaptureType::Multiple(v) => v.iter().collect(),
                    };
                    // По каждому тексту в `text_for_capture` мы будем искать совпадения
                    for text in &texts {
                        // Если есть простые подправила, то мы их проверяем
                        if let Some(simple_rules) = &frame
                            .0
                            .content_unchecked()
                            .subrules
                            .as_ref()
                            .unwrap()
                            .simple_rules
                        {
                            // пока что убрали RegexSet, из за счетчика contains для каждого `text` (только в этом режиме не используется RegexSet)
                            // 1 Этап
                            // Получаем все правила из simple_rules
                            for rule in simple_rules.all_rules.iter() {
                                trace!(
                                    "
                                    simple rule -  : (`{}`, `{:#?}`)",
                                    &rule.as_ref(),
                                    rule.content_unchecked().requirement
                                );
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(rule, text);
                                // Сразу узнаем, что будет дальше, если ошибка, то выходим из функции
                                if let NextStep::Error(value) =
                                    Self::next_or_data_for_error(rule, &mut captures)
                                {
                                    error!(
                                        "the rule (`{}`, `{:#?}`) did not work for text : `{}`",
                                        &rule.as_ref(),
                                        &rule.content_unchecked().requirement,
                                        text
                                    );
                                    return NextStep::Error(value);
                                }
                                // Загружаем во временный стек, если успех
                                temp_stack.push_back((rule, captures));
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
                                trace!(
                                    " complex rule -  : (`{}`, `{:#?}`)",
                                    &rule.as_ref(),
                                    rule.content_unchecked().requirement
                                );
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
                // Завершены все действия для правила
                NextStep::Finish => (),
                // Условие не сработало, значит ошибка
                NextStep::Error(value) => {
                    // ================= (LOG) =================
                    error!(
                        "the rule (`{}`, `{:#?}`) didn't work",
                        &frame.0.as_ref(),
                        &frame.0.content_unchecked().requirement,
                    );
                    // =========================================
                    return NextStep::Error(value);
                }
            }
        }
        // ================= (LOG) =================
        info!("for all matches all rules worked successfully");
        // =========================================
        // Финальный этап, мы загружаем всё в`stack` для дальнейшей обработки
        stack.extend(temp_stack.drain(..));
        NextStep::Finish
    }
}
