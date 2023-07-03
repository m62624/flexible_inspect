use super::*;

impl Rule {
    pub fn content_unchecked(&self) -> &TakeRuleForExtend {
        self.content.as_ref().expect(ERR_OPTION)
    }

    pub fn content_mut_unchecked(&mut self) -> &mut TakeRuleForExtend {
        self.content.as_mut().expect(ERR_OPTION)
    }

    pub fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize> {
        regex_set.matches(text).iter().collect::<Vec<_>>()
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
