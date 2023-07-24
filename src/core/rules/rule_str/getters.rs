use super::*;
use crate::core::rules::traits::RuleBase;

impl RuleBase for Rule {
    type TakeRuleType = TakeRuleForExtend;
    type SubRulesType = Subrules;
    /// Use for direct access to the structure body
    fn content_unchecked(&self) -> &Self::TakeRuleType {
        self.0.as_ref().expect(ERR_OPTION)
    }

    /// Use for direct access and modification to the body of the structure
    fn content_mut_unchecked(&mut self) -> &mut Self::TakeRuleType {
        self.0.as_mut().expect(ERR_OPTION)
    }
    fn get_subrules(&self) -> Option<&Self::SubRulesType> {
        self.content_unchecked().subrules.as_ref()
    }

    fn get_requirement(&self) -> &MatchRequirement {
        &self.content_unchecked().general_modifiers.requirement
    }

    fn get_counter(&self) -> Option<Counter> {
        self.content_unchecked().general_modifiers.counter
    }
}
