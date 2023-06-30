use std::sync::Arc;

use async_std::io::sink;

use super::rule::Rule;
use super::*;
impl CartridgeWrapper {
    pub async fn async_run(&self, text: &str) -> PyResult<()> {
        let mut tasks = Vec::new();
        let (text_for_task_1, text_for_task_2, text_for_task_3) =
            (text.to_owned(), text.to_owned(), text.to_owned());
        let (self_for_task_1, self_for_task_2, self_for_task_3) = (
            Arc::clone(&self.0),
            Arc::clone(&self.0),
            Arc::clone(&self.0),
        );

        tasks.push(async_std::task::spawn(async move {
            if let Some(simple_rules) = &self_for_task_1.root_rules.simple_rules {
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
        tasks.push(async_std::task::spawn(async move {
            if let Some(simple_rules) = &self_for_task_2.root_rules.simple_rules {
                let selected_rules =
                    Rule::get_selected_rules(&simple_rules.regex_set, &text_for_task_2);
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
        tasks.push(async_std::task::spawn(async move {
            if let Some(complex_rules) = &self_for_task_3.root_rules.complex_rules {
                complex_rules
                    .iter()
                    .map(|rule| Rule::run(&text_for_task_3, &rule, &self_for_task_3.py_class))
                    .collect::<PyResult<Vec<_>>>()?;
            }
            Ok::<(), PyErr>(())
        }));
        for task in tasks {
            task.await?;
        }
        Ok(())
    }
}
