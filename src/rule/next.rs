use super::captures::CaptureData;
use super::*;
use log::info;
/// Перечисление для определения следующего шага.
/// Используется вместо `bool`, для упрощения понимания кода,
/// любые новые модификаторы должны возвращать `NextStep`, тем самым
/// исопльзуем одну логику
#[derive(Debug, PartialEq, Eq)]
pub enum NextStep {
    /// `Go` - продолжить
    Go,
    /// `Finish` - завершить
    Finish,
    /// `Error` - ошибка
    Error(Option<HashMap<String, String>>),
}

impl Rule {
    /// Главный метод для получения разрешения запуска правила
    /// Все новые модификаторы, должны добавляться сюда
    pub fn next_or_data_for_error(rule: &rule::Rule, captures: &mut CaptureData) -> NextStep {
        // ================= (LOG) =================
        info!(
            "\nRule: (`{}`, `{:#?}`),\n`Captures: {:#?}`,\n",
            rule.as_ref(),
            rule.content_unchecked().requirement,
            captures.text_for_capture
        );
        // =========================================

        // Итак, что здесь происходит ?
        // Зависит от модификатора, мы либо идем дальше, либо завершаемся
        match rule.content_unchecked().requirement {
            // совпадение должно быть найдено
            MatchRequirement::MustBeFound => {
                match (
                    // резльутат совпадения
                    captures.is_some(),
                    // проверка присутствия подправил
                    rule.content_unchecked().subrules.is_some(),
                ) {
                    // Если есть совпадение и есть подправила, то мы идем дальше.
                    // Так как, если есть совпадение, и пользователь указал подправила,
                    // значит пользователь хочет, чтобы теперь на это совпадение
                    // были применены подправила
                    (true, true) => rule.counter_status(captures),
                    // Если есть совпадение, но нет подправил, то мы завершаемся.
                    (true, false) => {
                        if let NextStep::Error(value) = rule.counter_status(captures) {
                            return NextStep::Error(value);
                        }
                        NextStep::Finish
                    }
                    // Если нет совпадения, но есть подправила, это ошибка.
                    // Держим в голове, что пользователь указал подправила,
                    // а значит, должно было быть совпадение, на которое должны были
                    // сработать подправила
                    (false, true) => NextStep::Error(None),
                    // Если нет совпадения и нет подправил, возращаем ошибку.
                    // ведь оно должно было быть просто найдено, но условие не сработало
                    (false, false) => NextStep::Error(None),
                }
            }
            // совпадение не должно быть найдено
            MatchRequirement::MustNotBeFound => {
                match (
                    // резльутат совпадения
                    captures.is_some(),
                    // проверка присутствия подправил
                    rule.content_unchecked().subrules.is_some(),
                ) {
                    // Если есть совпадение и есть подправила, то мы завершаемся.
                    // Почему здесь иначе ?
                    // Так как у нас есть подправила, а значит, если есть совпадение,
                    // нужно произвести дополнительную проверку

                    // Но тогда зачем нам этот вариант в `MustNotBeFound`, если уже есть вариация
                    // в `MustBeFound` ?
                    // В `MustBeFound` если не найдено, это уже сразу ошибка
                    // А в `MustNotBeFound` если найдено, то без ошибки
                    //  |   |
                    // \|/ \|/
                    // К примеру, мы не хотим у себя найти, что то в файле, либо просто проверяем данные из кучи. Но выразить в обычный `regex`, будет довольно сложно
                    // поэтому можно сделать условие, сначала найти `A` (но оно не должно быть найдено),
                    // но так как мы указали подпрпавила, мы можем проверить совпадения из `A`, есть какие то нежелательные даннные в форме `Y`.
                    // Если же их нет, то можно не вызывать ошибку
                    (true, true) => rule.counter_status(captures),
                    // Если есть совпадение, но нет подправил, то мы вызываем ошибку
                    (true, false) => {
                        NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)))
                    }
                    // Если нет совпадения и есть подправила, то мы завершаемся.
                    (false, true) => {
                        if let NextStep::Error(value) = rule.counter_status(captures) {
                            return NextStep::Error(value);
                        }
                        NextStep::Finish
                    }
                    // Если нет совпадения и нет подправил, то мы завершаемся.
                    (false, false) => {
                        if let NextStep::Error(value) = rule.counter_status(captures) {
                            return NextStep::Error(value);
                        }
                        NextStep::Finish
                    }
                }
            }
        }
    }

    /// Проверка счетчика, если он есть
    /// Если совпадение больше или меньше, или равно по условию,
    /// то либо проходим дальше, либо вызываем ошибку
    fn counter_status(&self, captures: &mut CaptureData) -> NextStep {
        if let Some(value) = self.content_unchecked().counter {
            match value {
                // Если совпадений равно по условию, то проходим дальше
                Counter::Only(value) => {
                    // ================= (LOG) =================
                    info!(
                        "\nrule: `{}`,\nrule counter {:#?},\na total of {} matches found",
                        self.as_ref(),
                        self.content_unchecked().counter,
                        captures.counter_value
                    );
                    // =========================================

                    if captures.counter_value == value {
                        return NextStep::Go;
                    }
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
                // Если совпадений больше или равно по условию, то проходим дальше
                Counter::MoreThan(value) => {
                    // ================= (LOG) =================
                    info!(
                        "\nrule: `{}`,\nrule counter {:#?},\na total of {} matches found",
                        self.as_ref(),
                        self.content_unchecked().counter,
                        captures.counter_value
                    );
                    // =========================================
                    if captures.counter_value >= value {
                        return NextStep::Go;
                    }
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
                // Если совпадений меньше или равно по условию, то проходим дальше
                Counter::LessThan(value) => {
                    // ================= (LOG) =================
                    info!(
                        "\nrule: `{}`,\nrule counter {:#?},\na total of {} matches found",
                        self.as_ref(),
                        self.content_unchecked().counter,
                        captures.counter_value
                    );
                    // =========================================

                    if captures.counter_value <= value {
                        return NextStep::Go;
                    }
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
            }
        }
        // Если счетчика нет, то просто проходим дальше
        NextStep::Go
    }
}
