use super::*;

impl GeneralModifiers {
    pub fn new(requirement: MatchRequirement) -> Self {
        Self {
            requirement,
            counter: None,
            mod_match: ModeMatch::AllRulesForAllMatches,
        }
    }
}
