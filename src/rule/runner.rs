use super::captures::CaptureData;
use super::next::NextStep;
use super::*;
use std::collections::VecDeque;

impl Rule {
    /// Главный метод для итерационного запуска правила
    /// Проходит по всем подправилам, и так до самого конца для каждого `Rule`
    ///
    /// Этап 1
    /// У каждого правила, сначала идет проверка по `RegexSet`, наша задача
    /// вернуть ошибку валидаций, если хоть одно правило не прошло.
    /// А значит, в наших же интересах, чтобы сразу проверить, то что уже найдено
    /// через `RegexSet` (доп. инфа в `./rule/mod.rs`)
    /// Тем самым, мы стараемся максимально быстро  узнать резульат от самых простых правил
    /// (под простыми, имееются ввиду правила, которые сразу попали в `RegexSet`)
    ///
    /// Этап 2
    /// Дальше идут те правила которые не попали в `RegexSet`, но может так и должно быть.
    /// Ведь `RegexSet` отбирает те, что точно нашлись и только лишь потом проверяются их
    /// модификаторы. А не отобранные могут иметь те же модификтаоры MustBeFound и MustNotBefound, потому мы их проверяем во втором этапе
    ///
    /// Этап 3
    /// Дальше мы проверяем `FancyRegex`, в котором есть опережающие и ретроспективные проверки (lookaround & backreferences), такие правила могут дольше обрабатываться.
    /// Конечно, зависит от самого паттерна, но в целом, они могут быть дольше, чем обычные правила
    pub fn run(rule: &Rule, text: &str) -> NextStep {
        // Исопльзуем коллекцию для с двойным концом для итерационного запуска
        // Не исползьуем рекурсию, так в нашей вложенности нет ограничений.
        // Это значит, что каждое правило может содержать подправила, которые
        // могут содержать подправила, и т.д
        // Что значит, мы можем получить переполнение стека от одного же правила
        // Поэтому реализуем свои стек для каждого правила

        // сразу добавляем самое первое правило
        let mut stack = VecDeque::from([(rule, text)]);
        // теперь проходимся по стеку
        while let Some(stack_rule) = stack.pop_front() {
            // получаем совпадения по правилу,
            // держим в голове то, что каждое совпадение проверяется подправилом
            // а значит если совпадений несколько, и то каждое правило должно проверить каждое совпадение
            let mut captures = CaptureData::find_captures(stack_rule.0, stack_rule.1);
            // проверяем результат по условию (доп. инфа в `next.rs`)
            match Self::next_or_data_for_error(stack_rule.0, &mut captures) {
                // если все хорошо, то мы идем дальше
                NextStep::Go => {
                    // Если есть простые правила, идем дальше
                    if let Some(simple_rules) = &stack_rule
                        .0
                        .content_unchecked()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .simple_rules
                    {
                        // Начинаем сразу с `RegexSet` (1 - этап)
                        captures.text_for_capture.iter().for_each(|txt| {
                            Rule::get_selected_rules(&simple_rules.regex_set, txt)
                                .iter()
                                .for_each(|index| {
                                    // добавляем в стек, каждое правило с совпадением
                                    stack.push_back((&simple_rules.all_rules[*index], txt))
                                });
                        });
                        // Те правила, что не попали в `RegexSet` (2 - этап)
                        captures.text_for_capture.iter().for_each(|txt| {
                            simple_rules.all_rules.iter().for_each(|rule| {
                                if !stack.contains(&(rule, *txt)) {
                                    // добавляем в стек, каждое правило с совпадением
                                    stack.push_back((rule, txt));
                                }
                            });
                        });
                    }
                    // Если есть сложные правила, идем дальше
                    if let Some(complex_rules) = &stack_rule
                        .0
                        .content_unchecked()
                        .subrules
                        .as_ref()
                        .unwrap()
                        .complex_rules
                    {
                        // Проверяем сложные правила (3 - этап)
                        captures.text_for_capture.iter().for_each(|txt| {
                            complex_rules.iter().for_each(|rule| {
                                // добавляем в стек, каждое правило с совпадением
                                stack.push_back((rule, txt))
                            })
                        });
                    }
                }
                NextStep::Finish => (),
                NextStep::Error(value) => return NextStep::Error(value),
            }
        }
        NextStep::Finish
    }
}
