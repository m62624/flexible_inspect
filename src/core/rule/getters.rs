use super::*;

impl Rule {
    /// Получем содержимое из правило, без типа PyResult, так как пользователю
    /// необходимо знать, что правило потеряно из за модификатора,
    /// так как модификатор берет себе правило как владелец и возращает новое
    /// на основе него, с другими модфикаторами или же подправилами
    pub fn content_unchecked(&self) -> &TakeRuleForExtend {
        self.0.as_ref().expect(ERR_OPTION)
    }

    /// То же самое что и `content_unchecked`, только для изменения содержимого
    pub fn content_mut_unchecked(&mut self) -> &mut TakeRuleForExtend {
        self.0.as_mut().expect(ERR_OPTION)
    }

    /// Получаем отобранные правила из `RegexSet`
    pub fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize> {
        regex_set.matches(text).iter().collect()
    }
}

const ERR_OPTION: &str =
    "If you saved `Rule` in a variable, but used `extend` afterwards on the variable itself:
    
x = Rule(\"X\")
x.extend(Rule(\"Y\"))

* Please use this syntax:

x = Rule(\"X\").extend(Rule(\"Y\"))
* or 
x = Rule(\"X\")
x = x.extend(Rule(\"Y\"))";
