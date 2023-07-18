use super::super::*;

impl Rule {
    /*
    Используется `for` вместо `iterator`, так как возращаем `NextStep`, при
    использований цикла (`for`), мы можем сделать ранний выход из функции, если
    возникла ошибка
     */

    /// Проверяем, что хотя бы одно правило сработает на одно совпадение
    pub fn at_least_one_rule_for_at_least_one_match(
        stack: &mut VecDeque<(&Rule, CaptureData)>,
    ) -> NextStep {
        // Начнем проход по `stack`
        while let Some(mut frame) = stack.pop_front() {
            trace!("received frame from stack `{}`", frame.0.as_ref());
            // Проверяем, нужно ли идти дальше
            match Self::next_or_data_for_error(frame.0, &mut frame.1) {
                NextStep::Go => {
                    let texts: Vec<_> = match &frame.1.text_for_capture {
                        captures::CaptureType::Single(v) => v.iter().collect(),
                        captures::CaptureType::Multiple(v) => v.iter().collect(),
                    };
                    // Хранит ошибку, если она есть
                    let mut err: Option<HashMap<String, String>> = None;
                    // Статус, что нашли одно правило на одно совпадение
                    let mut found_rule = false;
                    // По каждому тексту в `text_for_capture` мы будем искать совпадение
                    'stop_text: for text in texts {
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
                            'skip_rule: for index in
                                Rule::get_selected_rules(&simple_rules.regex_set, text)
                            {
                                trace!(
                                    "retrieved rules from `RegexSet` : `{}`",
                                    &simple_rules.all_rules[index].as_ref()
                                );
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(
                                    &simple_rules.all_rules[index],
                                    text,
                                );
                                // Проверяем это правило
                                if let NextStep::Error(value) = Self::next_or_data_for_error(
                                    &simple_rules.all_rules[index],
                                    &mut captures,
                                ) {
                                    err = value;
                                    continue 'skip_rule;
                                }
                                // ================= (LOG) =================
                                info!(
                                    "\nfound one rule (`{}`) \nfor one match (`{:#?}`)",
                                    &simple_rules.all_rules[index].as_ref(),
                                    captures.text_for_capture
                                );
                                // =========================================
                                // Помечаем, что нашли правило
                                found_rule = true;
                                // Загружаем во временный стек если успех
                                stack.push_back((&simple_rules.all_rules[index], captures));
                                break 'stop_text;
                            }
                            // 2 Этап
                            // Получаем правила, которые не попали в `RegexSet`
                            'skip_rule: for rule in simple_rules.all_rules.iter() {
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(rule, text);
                                // Проверяем, что мы не обрабатывали это правило ранее
                                if !stack.iter().any(|&(r, _)| r == rule) {
                                    trace!(
                                        "received rules that are not in `RegexSet` : `{}`",
                                        &rule.as_ref()
                                    );
                                    // Проверяем это правило
                                    if let NextStep::Error(value) =
                                        Self::next_or_data_for_error(rule, &mut captures)
                                    {
                                        err = value;
                                        continue 'skip_rule;
                                    }
                                    // ================= (LOG) =================
                                    info!(
                                        "found one rule (`{}`) for one match (`{:#?}`)",
                                        rule.as_ref(),
                                        captures.text_for_capture
                                    );
                                    // =========================================

                                    // Помечаем, что нашли правило
                                    found_rule = true;
                                    // Загружаем во временный стек, если успех
                                    stack.push_back((rule, captures));
                                    break 'stop_text;
                                }
                            }
                        }
                        if !found_rule {
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
                                'skip_rule: for rule in complex_rules {
                                    // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                    let mut captures = CaptureData::find_captures(rule, text);
                                    // Проверяем это правило
                                    if let NextStep::Error(value) =
                                        Self::next_or_data_for_error(rule, &mut captures)
                                    {
                                        err = value;
                                        continue 'skip_rule;
                                    }
                                    // Помечаем, что нашли правило
                                    found_rule = true;
                                    // Загружаем во временный стек если успех
                                    stack.push_back((rule, captures));
                                    break 'stop_text;
                                }
                            }
                        }
                    }
                    if !found_rule {
                        // ================= (LOG) =================
                        error!("no rules were found for any of the matches");
                        // =========================================
                        return NextStep::Error(err);
                    }
                }
                // Завершены все действия для правила
                NextStep::Finish => (),
                // Условие не сработало, значит ошибка
                NextStep::Error(value) => {
                    // ================= (LOG) =================
                    error!("no rules were found for any of the matches");
                    // =========================================
                    return NextStep::Error(value);
                }
            }
        }
        NextStep::Finish
    }
}
