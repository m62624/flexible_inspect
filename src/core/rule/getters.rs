use super::*;

impl Rule {
    /// Use for direct access to the structure body
    pub(crate) fn content_unchecked(&self) -> &TakeRuleForExtend {
        self.0.as_ref().expect(ERR_OPTION)
    }

    /// Use for direct access and modification to the body of the structure
    pub(crate) fn content_mut_unchecked(&mut self) -> &mut TakeRuleForExtend {
        self.0.as_mut().expect(ERR_OPTION)
    }

    /// Get selected rules from `RegexSet`
    pub(crate) fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize> {
        regex_set.matches(text).iter().collect()
    }
}

const ERR_OPTION: &str = " The body of `Rule` is missing, maybe you used modifiers, they borrow `Rule`, modifiers modify it and return the already modified version";
