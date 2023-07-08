use super::*;

impl Rule {
    /*
    Используется `for` вместо `iterator`, так как возращаем `NextStep`, при
    использований цикла (`for`), мы можем сделать ранний выход из функции, если
    возникла ошибка
     */

    /// Проверяем, что хотя бы одно правило должно сработать для всех совпадений
    pub fn at_least_one_rule_for_all_matches(
        stack: &mut VecDeque<(&Rule, CaptureData)>,
    ) -> NextStep {
        // Создаем временный стек, в который будем складывать все правила, которые нужно обработать
        let mut temp_stack: VecDeque<(&Rule, CaptureData)> = VecDeque::new();
        // нужно для проверки одного индекса из сета индексов `RegexSet`,
        // от каждого совпадения, мы получаем `RegexSet` (vec<usize>),
        // и считаем сколько раз встречается каждый индекс, если он встречается
        // столько же раз, сколько и совпадений, значит это правило подходит
        let mut counter_one_rule = HashMap::new();
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
            // Проверяем, нужно ли идти дальше
            match Self::next_or_data_for_error(frame.0, &mut frame.1) {
                NextStep::Go => {
                    // Статус, нашли ли мы одно правило для всех совпадений
                    let mut one_rule_found = false;
                    // Хранит ошибку, если она есть
                    let mut err: Option<HashMap<String, String>> = None;
                    // Если есть простые подправила, то мы их проверяем
                    if let Some(simple_rules) = &frame
                        .0
                        .content_unchecked()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .simple_rules
                    {
                        // Проходимся по всем совпадениям
                        'skip_text: for text in frame.1.text_for_capture.iter() {
                            // Проходимся по всем индексам, которые вернул `RegexSet`
                            'skip_this_rule: for index in
                                Rule::get_selected_rules(&simple_rules.regex_set, text)
                            {
                                // Если индекс уже есть в `counter_one_rule`, то мы его увеличиваем
                                *counter_one_rule.entry(index).or_insert(0) += 1;
                                // сверяем, сколько раз встретился индекс, с количеством совпадений
                                if counter_one_rule[&index] == frame.1.text_for_capture.len() {
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
                                        continue 'skip_this_rule;
                                    }

                                    // ================ (LOG) =================
                                    info!(
                                        "found one rule for all matches: {}",
                                        &simple_rules.all_rules[index].as_ref()
                                    );
                                    // =========================================

                                    // Если мы дошли до этого момента, значит мы нашли правило
                                    one_rule_found = true;
                                    // Если все хорошо, то добавляем в стек
                                    temp_stack
                                        .push_back((&simple_rules.all_rules[index], captures));
                                    // Выходим из цикла, так как мы нашли правило
                                    break 'skip_text;
                                }
                            }
                        }
                        // На всякий случай, очищаем `counter_one_rule`, после проверки простых правил
                        counter_one_rule.clear();
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
                        // Проверяем не нашли ли мы уже правило
                        if !one_rule_found {
                            // Проходимся по всем правилам
                            'main_complex: for rule in complex_rules {
                                // Каждое правило проверяет каждое совпадение
                                for text in frame.1.text_for_capture.iter() {
                                    // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                    let mut captures = CaptureData::find_captures(rule, text);
                                    // Проверяем это правило
                                    if let NextStep::Error(value) =
                                        Self::next_or_data_for_error(rule, &mut captures)
                                    {
                                        err = value;
                                        // Если ошибка, то переходим к следующему правилу
                                        continue 'main_complex;
                                    }

                                    // ================ (LOG) =================
                                    info!("found one rule for all matches: {}", &rule.as_ref());
                                    // =========================================

                                    // Если все хорошо, то добавляем в стек
                                    temp_stack.push_back((rule, captures));
                                }
                                // Если мы дошли до этого момента, значит мы нашли правило
                                one_rule_found = true;
                            }
                        }
                    }
                    // Если мы нашли правило, то добавляем в стек
                    if one_rule_found {
                        stack.extend(temp_stack.drain(..));
                    } else {
                        // ================= (LOG) =================
                        error!("not found one rule for all matches");
                        // =========================================

                        // Если мы не нашли правило, то возвращаем ошибку
                        return NextStep::Error(err);
                    }
                }
                // Завершены все действия для правила
                NextStep::Finish => (),
                // Условие не сработало, значит ошибка
                NextStep::Error(value) => return NextStep::Error(value),
            }
        }
        NextStep::Finish
    }
}
