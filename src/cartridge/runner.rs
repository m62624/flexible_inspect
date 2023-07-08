use super::rule::{next::NextStep, Rule};
use super::*;
use async_std::task;
use log::info;

/// Первые варианты, использовали итераторы,
/// но проблема в том, что даже через `All`, нужно передать ещё и результат ошибки
/// Поэтому используется `loop for`

/*
   если взять `iterator`, тогда пришлось бы
   проверять все элементы через `Rule::run` и только потом проверять `NextStep::Error(value)`, в `for` сразу проходят проверка `if`, так мы получим `NextStep::Error(value)` для достижения раннего возврата
*/
impl CartridgeWrapper {
    /// Запускает все правила, которые есть в классе\
    /// Описание этапов и принципов работы опиcаны в `./rule/runner.rs`
    pub fn sync_run(&self, text: &str) -> NextStep {
        // ================= (LOG) =================
        info!(
            "iteratively pass by the rules of the `{}` class",
            self.0.get_py_class().to_string()
        );
        //==========================================

        // Проверка простых правил
        if let Some(simple_rules) = &self.0.root_rules.simple_rules {
            let selected_rules = Rule::get_selected_rules(&simple_rules.regex_set, text);

            // Правила отобранные из regexset
            for index in &selected_rules {
                if let NextStep::Error(value) = Rule::run(&simple_rules.all_rules[*index], text) {
                    return NextStep::Error(value);
                }
            }

            // Правила которые не попали в regexset
            for (index, _) in simple_rules.all_rules.iter().enumerate() {
                if !selected_rules.contains(&index) {
                    if let NextStep::Error(value) = Rule::run(&simple_rules.all_rules[index], text)
                    {
                        return NextStep::Error(value);
                    }
                }
            }
        }

        // Проверка сложных правил
        if let Some(complex_rules) = &self.0.root_rules.complex_rules {
            for rule in complex_rules {
                if let NextStep::Error(value) = Rule::run(rule, text) {
                    return NextStep::Error(value);
                }
            }
        }
        NextStep::Finish
    }

    /// Запускает все правила, которые есть в классе\
    /// Описание этапов и принципов работы опиcаны в `./rule/runner.rs`
    pub async fn async_run(&self, text: Arc<String>) -> NextStep {
        // ================= (LOG) =================
        info!(
            "iteratively pass by the rules of the `{}` class",
            self.0.get_py_class().to_string()
        );
        //==========================================

        // Коллекция для конкурентного выполнения задач
        let mut tasks = Vec::new();

        // Повышаем счетчик ссылок на `text`, для каждой задачи
        let text_1 = Arc::clone(&text);
        let text_2 = Arc::clone(&text);
        let text_3 = Arc::clone(&text);

        // Повышаем счетчик ссылок на `self`, для каждой задачи
        let self_for_task_1 = Arc::clone(&self.0);
        let self_for_task_2 = Arc::clone(&self.0);
        let self_for_task_3 = Arc::clone(&self.0);

        // Если есть простые правила, идем дальше
        if let Some(self_simple_rules) = &self.0.root_rules.simple_rules {
            let selected_rules = Arc::new(Rule::get_selected_rules(
                &self_simple_rules.regex_set,
                &text,
            ));
            // Правила отобранные из `RegexSet` для task 1
            let selected_rules_1 = Arc::clone(&selected_rules);
            // Правила отобранные из `RegexSet` для task 2
            let selected_rules_2 = Arc::clone(&selected_rules);

            // Создаем задачу для отобранных правил из `RegexSet`
            tasks.push(task::spawn(async move {
                for index in &*selected_rules_1 {
                    if let NextStep::Error(value) = Rule::run(
                        &self_for_task_1
                            .root_rules
                            .simple_rules
                            .as_ref()
                            .unwrap()
                            .all_rules[*index],
                        &text_1,
                    ) {
                        return NextStep::Error(value);
                    }
                }
                NextStep::Finish
            }));

            // Создаем задачу для правил, которые не попали в `RegexSet`
            tasks.push(task::spawn(async move {
                for (index, _) in self_for_task_2
                    .root_rules
                    .simple_rules
                    .as_ref()
                    .unwrap()
                    .all_rules
                    .iter()
                    .enumerate()
                {
                    if !selected_rules_2.contains(&index) {
                        if let NextStep::Error(value) = Rule::run(
                            &self_for_task_2
                                .root_rules
                                .simple_rules
                                .as_ref()
                                .unwrap()
                                .all_rules[index],
                            &text_2,
                        ) {
                            return NextStep::Error(value);
                        }
                    }
                }
                NextStep::Finish
            }));
        }
        // Создаем задачу для сложных правил
        tasks.push(task::spawn(async move {
            if let Some(complex_rules) = &self_for_task_3.root_rules.complex_rules {
                for rule in complex_rules {
                    if let NextStep::Error(value) = Rule::run(rule, &text_3) {
                        return NextStep::Error(value);
                    }
                }
            }
            NextStep::Finish
        }));

        // Запускаем все задачи, и ждем их завершения
        // Если хоть одна задача вернет ошибку, то мы сразу возвращаем ее
        for task in tasks {
            if let NextStep::Error(value) = task.await {
                return NextStep::Error(value);
            }
        }
        NextStep::Finish
    }
}
