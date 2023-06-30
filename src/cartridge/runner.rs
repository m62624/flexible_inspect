use super::rule::Rule;
use super::*;

impl CartridgeWrapper {
    /// Синхронный запуск
    pub fn sync_run(&self, text: &str) -> PyResult<()> {
        if let Some(simple_rules) = &self.0.root_rules.simple_rules {
            // Отбираем простые правила через `RegexSet`
            let selected_rules = Rule::get_selected_rules(&simple_rules.regex_set, &text);

            // Создаем задачи 3 задачи
            // 1 - простые правила, отобранные через `RegexSet`
            // 2 - простые правила, но не выбранные в первом этапе
            // 3 - сложные правила

            // 1
            selected_rules
                .iter()
                .map(|id| Rule::run(&text, &simple_rules.all_rules[*id], &self.0.py_class))
                .collect::<PyResult<()>>()?;

            // 2
            simple_rules
                .all_rules
                .iter()
                .enumerate()
                .filter(|(id, _)| !selected_rules.contains(id))
                .map(|(_, rule)| Rule::run(&text, &rule, &self.0.py_class))
                .collect::<PyResult<Vec<_>>>()?;
        }

        // 3
        if let Some(complex_rules) = &self.0.root_rules.complex_rules {
            complex_rules
                .iter()
                .map(|rule| Rule::run(&text, &rule, &self.0.py_class))
                .collect::<PyResult<Vec<_>>>()?;
        }
        Ok(())
    }

    /// Асинхронный запуск
    pub async fn async_run(&self, text: &str) -> PyResult<()> {
        // Коллекция для конкурентного выполнения задач
        let mut tasks: Vec<async_std::task::JoinHandle<Result<(), PyErr>>> = Vec::new();
        // Создаем копии текста для каждой задачи
        let (text_for_task_1, text_for_task_2, text_for_task_3) =
            (text.to_string(), text.to_string(), text.to_string());
        // Создаем копии на ссылку `self` для каждой задачи
        let (self_for_task_1, self_for_task_2, self_for_task_3) = (
            Arc::clone(&self.0),
            Arc::clone(&self.0),
            Arc::clone(&self.0),
        );

        // Создаем задачи 3 задачи
        // 1 - простые правила, отобранные через `RegexSet`
        // 2 - простые правила, но не выбранные в первом этапе
        // 3 - сложные правила

        // 1
        tasks.push(async_std::task::spawn(async move {
            if let Some(simple_rules) = &self_for_task_1.root_rules.simple_rules {
                // Отбираем простые правила через `RegexSet`
                let selected_rules =
                    Rule::get_selected_rules(&simple_rules.regex_set, &text_for_task_1);
                selected_rules
                    .iter()
                    .map(|id| {
                        Rule::run(
                            &text_for_task_1,
                            &simple_rules.all_rules[*id],
                            &self_for_task_1.py_class,
                        )
                    })
                    .collect::<PyResult<()>>()?;
            }
            Ok::<(), PyErr>(())
        }));

        // 2
        tasks.push(async_std::task::spawn(async move {
            if let Some(simple_rules) = &self_for_task_2.root_rules.simple_rules {
                // Отбираем простые правила через `RegexSet`
                let selected_rules =
                    Rule::get_selected_rules(&simple_rules.regex_set, &text_for_task_2);
                // Исклчаем уже отобранные правила
                simple_rules
                    .all_rules
                    .iter()
                    .enumerate()
                    .filter(|(id, _)| !selected_rules.contains(id))
                    .map(|(_, rule)| Rule::run(&text_for_task_2, &rule, &self_for_task_2.py_class))
                    .collect::<PyResult<Vec<_>>>()?;
            }
            Ok::<(), PyErr>(())
        }));

        // 3
        tasks.push(async_std::task::spawn(async move {
            if let Some(complex_rules) = &self_for_task_3.root_rules.complex_rules {
                complex_rules
                    .iter()
                    .map(|rule| Rule::run(&text_for_task_3, &rule, &self_for_task_3.py_class))
                    .collect::<PyResult<Vec<_>>>()?;
            }
            Ok::<(), PyErr>(())
        }));
        // Ожидаем завершения всех задач
        for task in tasks {
            task.await?;
        }
        Ok(())
    }
}
