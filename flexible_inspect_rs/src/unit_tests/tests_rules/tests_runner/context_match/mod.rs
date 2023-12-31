mod tests_all_rules_for_all_matches;
mod tests_all_rules_for_at_least_one_match;
mod tests_at_least_one_rule_for_all_matches;
mod tests_at_least_one_rule_for_at_least_one_match;

use crate::prelude::*;
use crate::rules::traits::CalculateValueRules;
use crate::rules::{self, next::NextStep};
use std::collections::HashMap;
