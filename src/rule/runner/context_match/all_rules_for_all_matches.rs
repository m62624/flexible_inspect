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
        // Начнем проход по `stack`, `stack_temp` будет расширять `stack`
        while let Some((rule, capture_data)) = stack.pop_front() {
            
        }
        // ================= (LOG) =================
        info!("for all matches all rules worked successfully");
        // =========================================
        // Финальный этап, мы загружаем всё в`stack` для дальнейшей обработки
        stack.extend(temp_stack.drain(..));
        NextStep::Finish
    }
}
