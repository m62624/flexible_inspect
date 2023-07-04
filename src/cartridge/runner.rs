use super::rule::{next::NextStep, Rule};
use super::*;
use async_std::task;
/*
   можно было бы использовать `iterator`, но тогда пришлось бы
   проверять все элементы через `Rule::run` и только потом проверять `NextStep::Error(value)`, хоть и `for` использует `iterator` но в нем сразу проходят проверка `if`,
   так мы сразу проверяем `NextStep::Error(value)` для достижения раннего возврата

*/
impl CartridgeWrapper {
    pub fn sync_run(&self, text: &str) -> NextStep {
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

    pub async fn async_run(&self, text: Arc<String>) -> NextStep {
        // Коллекция для конкурентного выполнения задач
        let mut tasks = Vec::new();

        // Клонируем текст, чтобы обеспечить 'static lifetime ссылки
        let text_1 = Arc::clone(&text);
        let text_2 = Arc::clone(&text);
        let text_3 = Arc::clone(&text);

        // Создаем копии на ссылку `self` для каждой задачи
        let self_for_task_1 = Arc::clone(&self.0);
        let self_for_task_2 = Arc::clone(&self.0);
        let self_for_task_3 = Arc::clone(&self.0);

        if let Some(self_simple_rules) = &self.0.root_rules.simple_rules {
            let selected_rules = Arc::new(Rule::get_selected_rules(
                &self_simple_rules.regex_set,
                &text,
            ));
            let selected_rules_1 = Arc::clone(&selected_rules);
            let selected_rules_2 = Arc::clone(&selected_rules);

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

        for task in tasks {
            if let NextStep::Error(value) = task.await {
                return NextStep::Error(value);
            }
        }
        NextStep::Finish
    }
}
