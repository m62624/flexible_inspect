use super::*;

impl Rule {
    /// Проверка счетчика, если он есть
    /// Если совпадение больше или меньше, или равно по условию,
    /// то либо проходим дальше, либо вызываем ошибку
    pub fn counter_status(&self, captures: &mut CaptureData) -> NextStep {
        if let Some(value) = self.content_unchecked().counter {
            match value {
                // Если совпадений равно по условию, то проходим дальше
                Counter::Only(value) => {
                    // ================= (LOG) =================
                    info!(
                        "\nTHE RESULT: \nrule: `{}`,\nrule counter {:#?},\na total of {} matches found",
                        self.as_ref(),
                        self.content_unchecked().counter,
                        captures.counter_value
                    );
                    // =========================================

                    if captures.counter_value == value {
                        return NextStep::Go;
                    }
                    // ================= (LOG) =================
                    error!(
                        "for the `{}` rule must be matched: `{:#?}`\ntotal matches found: `{}`",
                        self.as_ref(),
                        self.content_unchecked().counter,
                        captures.counter_value,
                    );
                    // =========================================
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
                // Если совпадений больше или равно по условию, то проходим дальше
                Counter::MoreThan(value) => {
                    // ================= (LOG) =================
                    info!(
                        "\nTHE RESULT: \nrule: `{}`,\nrule counter {:#?},\na total of {} matches found",
                        self.as_ref(),
                        self.content_unchecked().counter,
                        captures.counter_value
                    );
                    // =========================================
                    if captures.counter_value >= value {
                        return NextStep::Go;
                    }
                    // ================= (LOG) =================
                    error!(
                        "for the `{}` rule must be matched: `{:#?}`\ntotal matches found: `{}`",
                        self.as_ref(),
                        self.content_unchecked().counter,
                        captures.counter_value,
                    );
                    // =========================================
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
                // Если совпадений меньше или равно по условию, то проходим дальше
                Counter::LessThan(value) => {
                    // ================= (LOG) =================
                    info!(
                        "\nTHE RESULT: \nrule: `{}`,\nrule counter {:#?},\na total of {} matches found",
                        self.as_ref(),
                        self.content_unchecked().counter,
                        captures.counter_value
                    );
                    // =========================================

                    if captures.counter_value <= value {
                        return NextStep::Go;
                    }
                    // ================= (LOG) =================
                    error!(
                        "for the `{}` rule must be matched: `{:#?}`\ntotal matches found: `{}`",
                        self.as_ref(),
                        self.content_unchecked().counter,
                        captures.counter_value,
                    );
                    // =========================================
                    return NextStep::Error(Some(std::mem::take(&mut captures.hashmap_for_error)));
                }
            }
        }
        // Если счетчика нет, то просто проходим дальше
        NextStep::Finish
    }
}
