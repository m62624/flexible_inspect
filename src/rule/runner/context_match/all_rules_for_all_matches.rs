use super::*;

impl Rule {
    /*
    Используется `for` вместо `iterator`, так как возращаем `NextStep`, при
    использований цикла (`for`), мы можем сделать ранний выход из функции, если
    возникла ошибка
     */
    /// Проверяем, что на каждое совпадение, точно сработают все правила
    pub fn all_rules_for_all_matches(stack: &mut VecDeque<(&Rule, CaptureData)>) -> NextStep {
        // Создаем временный стек, в который будем складывать все правила, которые нужно обработать, если они проходят проверку
        let mut temp_stack: VecDeque<(&Rule, CaptureData)> = VecDeque::new();
        // Начнем проход по стеку, который мы получили в параметрах функцмй, `stack_temp` будет расширять `stack` который мы получили сверху, при выполнении условий
        while let Some(mut frame) = stack.pop_front() {
            // Проверяем, что мы можем продолжить выполнение правила, если нет, то либо пропуск,либо ошибка
            match Self::next_or_data_for_error(frame.0, &mut frame.1) {
                NextStep::Go => {
                    // Если мы в `Go`, это значит, что есть `subrules`, но какие именно мы не знаем
                    // Проверяем, что есть `simple_rules`, если есть, то начинаем их обработку
                    if let Some(simple_rules) = &frame
                        .0
                        .content_unchecked()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .simple_rules
                    {
                        // по каждому тексту в `text_for_capture` мы будем искать совпадения
                        for text in frame.1.text_for_capture.iter() {
                            // получаем `Rule` которые прошли
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
                                    return NextStep::Error(value);
                                }
                                // Загружаем во временный стек если успех
                                temp_stack.push_back((&simple_rules.all_rules[index], captures));
                            }
                        }
                        // Если при прошлом цикле, мы не получили не одной ошибки, то мы можем загрузить эти правила в стек, держим в голове, что наша условие :
                        // `all_rules_for_all_matches` - проверяем, что на каждое совпадение, точно сработают все правила
                        // а значит мы загружаем их лишь тогда, когда успех идет по всем правилам
                        // Здесь же, мы загружаем сразу, так как дальше нужно узнать, какие
                        // правила не попали в первом цикле, ведь даже если дальше пройдет ошибка, мы можем не беспокойтся о том, что мы загрузили сейчас.
                        // Ведь мы сразу вернем `NextStep::Error`, и стек функций уже не будет проверен
                        stack.extend(temp_stack.drain(..));

                        // Теперь проходимся по тому, что не попало в `regex_set`
                        for text in frame.1.text_for_capture.iter() {
                            for rule in simple_rules.all_rules.iter() {
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(rule, text);
                                // Проверяем, что мы не обрабатывали это правило ранее
                                if !stack.iter().any(|&(r, _)| r == rule) {
                                    dbg!(rule);
                                    // Сразу узнаем, что будет дальше, если ошибка, то выходим из функции
                                    if let NextStep::Error(value) =
                                        Self::next_or_data_for_error(rule, &mut captures)
                                    {
                                        return NextStep::Error(value);
                                    }
                                    // Загружаем во временный стек если успех
                                    temp_stack.push_back((rule, captures));
                                }
                            }
                        }
                    }
                    // Теперь проверяем, что есть `complex_rules`, если есть, то начинаем их обработку
                    if let Some(complex_rules) = &frame
                        .0
                        .content_unchecked()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .complex_rules
                    {
                        // по каждому тексту в `text_for_capture` мы будем искать совпадения
                        for text in frame.1.text_for_capture.iter() {
                            // получаем complex_rules
                            for rule in complex_rules {
                                // Сохраняем в отдельной переменой, чтобы не дублировать данные
                                let mut captures = CaptureData::find_captures(rule, text);
                                // Сразу узнаем, что будет дальше, если ошибка, то выходим из функции
                                if let NextStep::Error(value) =
                                    Self::next_or_data_for_error(rule, &mut captures)
                                {
                                    return NextStep::Error(value);
                                }
                                // Загружаем во временный стек если успех
                                temp_stack.push_back((rule, captures));
                            }
                        }
                    }
                    // Загружаем в стек, если успех
                    stack.extend(temp_stack.drain(..));
                }
                NextStep::Finish => (),
                NextStep::Error(value) => return NextStep::Error(value),
            }
        }
        NextStep::Finish
    }
}

