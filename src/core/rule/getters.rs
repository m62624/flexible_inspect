use super::*;

impl Rule {
    /// Получем содержимое из правило, без типа PyResult, так как пользователю
    /// необходимо знать, что правило потеряно из за модификатора,
    /// так как модификатор берет себе правило как владелец и возращает новое
    /// на основе него, с другими модфикаторами или же подправилами
    pub(crate) fn content_unchecked(&self) -> &TakeRuleForExtend {
        self.0.as_ref().expect(ERR_OPTION)
    }

    /// То же самое что и `content_unchecked`, только для изменения содержимого
    pub(crate) fn content_mut_unchecked(&mut self) -> &mut TakeRuleForExtend {
        self.0.as_mut().expect(ERR_OPTION)
    }

    /// Получаем отобранные правила из `RegexSet`
    pub(crate) fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize> {
        regex_set.matches(text).iter().collect()
    }
}

const ERR_OPTION: &str = " The body of `Rule` is missing, maybe you used modifiers, they borrow `Rule`, modifiers modify it and return the already modified version";
