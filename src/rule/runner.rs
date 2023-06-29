use super::captures::MultiCapture;
use super::validate::actions_from_the_requirement::next_or_error;
use super::*;
use std::collections::VecDeque;

impl Rule {
    /*
    Раннер для одной проверки из корня.
    Это функция принимает корень правила, и идет до самого низа, проверяя каждое правило
     */
    pub fn run(py: Python, text: &str, rule: &Rule, class_template: &PyObject) -> PyResult<()> {
        // Используем стек для каждого правила
        // Если использовать рекурсию, то будет переполнение стека от одного же правила
        let mut stack = VecDeque::from([(rule, text)]);
        // Пока стек не пустой выполняем проверку правил
        while let Some(stack_rule) = stack.pop_back() {
            // Получаем все совпадения с корня
            let captures = MultiCapture::find_captures(stack_rule.0, stack_rule.1)?;
            // dbg!(&captures);
            // Если совпадения найдены, проверяем следующее правило (уже начинается итерация до глубины)
            // Если уже корневое правило не пройдено, то итерация не продолжается, сразу происходит проброс ошибки из `next_or_error`
            // (если next_or_error вернет `true`, это дает гарантию,
            // что `Subrules` не пустой и можно использовать `unchecked` + `unwrap`)
            if next_or_error(py, class_template, stack_rule.0, &captures)? {
                // Получаем все совпадения с корня, и используем их для проверки следующего правила (всех дочерних вложенных правил с подправилами)
                let texts = captures.to_str_vec();
                // Два этапа проверки :
                // 1) Проверка самых простых правил (`default` + `regexSet`)
                // 2) Проверка самых сложных правил (`fancy`)
                // Первый этап, проверяем самые простые правила
                if let Some(rgxs_set) = &stack_rule.0.unchacked_get_rgx_set() {
                    // Каждый текст проверяем на совпадение с каждым правилом
                        texts
                        .iter()
                        .map(|text| {
                            // dbg!(text);
                            // Получаем индексы совпадений с regexSet
                            Rule::get_selected_rules(rgxs_set, text)
                                .iter()
                                .map(|id| {
                                    // dbg!(&id);
                                    // Для каждого совпадения добавляем в стек проверку следующего правила
                                    stack.push_back((
                                        &stack_rule.0.unchacked_get_rgx_vec()[*id],
                                        text,
                                    ));
                                })
                                .for_each(drop);
                            // Каждый текст проверяем на совпадение с каждым правилом
                            texts
                                .iter()
                                .map(|text| {
                                    stack_rule
                                        .0
                                        .unchacked_get_rgx_vec()
                                        .iter()
                                        .map(|rule| {
                                            // Теперь добавляем правила из `default` которые не попали в `regexSet`
                                            if !stack.contains(&(rule, text)) {
                                                stack.push_back((rule, text));
                                            }
                                        })
                                        .for_each(drop);
                                })
                                .for_each(drop);
                        })
                        .for_each(drop);
                }

                // Если первый этап пройден, переходим ко второму этапу, проверка на существование сложных правил (`fancy`)
                if let Some(f_r) = stack_rule.0.subrules.as_ref().unwrap().get_fancy_rgx_vec() {
                    // Каждый текст проверяем на совпадение с каждым правилом
                    texts
                        .iter()
                        .map(|text| {
                            f_r.iter()
                                .map(|rules| stack.push_back((rules, text)))
                                .for_each(drop);
                        })
                        .for_each(drop);
                }
            }
        }
        Ok(())
    }
}
