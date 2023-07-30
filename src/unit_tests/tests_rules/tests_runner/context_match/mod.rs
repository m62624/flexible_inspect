mod tests_all_rules_for_all_matches;
mod tests_all_rules_for_at_least_one_match;
mod tests_at_least_one_rule_for_all_matches;
mod tests_at_least_one_rule_for_at_least_one_match;

use crate::core::rules::traits::CalculateValueRules;
use crate::core::rules::{self, next::NextStep};
use crate::prelude::*;
use std::collections::HashMap;
