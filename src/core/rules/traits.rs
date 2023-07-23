pub trait RuleModifier {}

pub trait RuleBase {
    type TakeRuleType;
    fn content_unchecked(&self) -> &Self::TakeRuleType;
    fn content_mut_unchecked(&mut self) -> &mut Self::TakeRuleType;
}

pub trait RuleExtendBase: RuleBase {
    fn get_selected_rules(regex_set: &regex::RegexSet, text: &str) -> Vec<usize>;
}

pub trait RuleBytesExtendBase: RuleBase {
    fn get_selected_rules(regex_set: &regex::bytes::RegexSet, text_bytes: &[u8]) -> Vec<usize>;
}
