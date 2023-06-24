use super::*;
impl Rule {
    /// Получение строки и Regex типа строки из `Rule`
    pub fn get_inner(
        &self,
    ) -> &Option<(
        Box<str>,
        regex_types::RGX,
        match_requirement::MatchRequirement,
    )> {
        &self.rule_raw
    }
    /// Получение строки (`Regex`) из `Rule`
    pub fn get_str(&self) -> PyResult<&Box<str>> {
        if let Some(value) = self.get_inner() {
            return Ok(&value.0);
        }
        return Err(error_take());
    }
    pub fn get_type(&self) -> PyResult<&regex_types::RGX> {
        if let Some(value) = &self.get_inner() {
            return Ok(&value.1);
        }
        return Err(error_take());
    }
    /// Получение `MatchRequirement` из `Rule`
    pub fn get_requirement(&self) -> PyResult<&MatchRequirement> {
        if let Some(value) = &self.get_inner() {
            return Ok(&value.2);
        }
        return Err(error_take());
    }
    pub fn get_subrules(&self) -> &Option<Vec<Rule>> {
        &self.rules_for_the_rule
    }
    /// Проверка на существование подправил
    pub fn is_exist_subrules(&self) -> bool {
        self.rules_for_the_rule.is_some()
    }
}

/// Если в `Rule` есть поле в котором находится None, то возвращается ошибка владения
fn error_take() -> PyErr {
    PyErr::new::<exceptions::PyValueError, _>(format!(
        "* If you saved `Rule` in a variable, but used `extend` afterwards on the variable itself:

        x = Rule(\"x\")
        x.extend(Rule(\"Y\"))
        
        * Please use this syntax:
        
        x = Rule(\"x\").extend(Rule(\"y\"))
        * or 
        x = Rule(\"x\")
        x = x.extend(Rule(\"y\"))"
    ))
}
