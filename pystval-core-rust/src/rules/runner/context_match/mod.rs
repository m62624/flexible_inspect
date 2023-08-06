// =======================================================
mod all_rules_for_all_matches;
mod all_rules_for_at_least_one_match;
mod at_least_one_rule_for_all_matches;
mod at_least_one_rule_for_at_least_one_match;
// =======================================================
use crate::rules::traits::RuleBase;
use crate::rules::{next::NextStep, traits::CalculateValueRules, CaptureData};
pub use all_rules_for_all_matches::all_rules_for_all_matches;
pub use all_rules_for_at_least_one_match::all_rules_for_at_least_one_match;
pub use at_least_one_rule_for_all_matches::at_least_one_rule_for_all_matches;
pub use at_least_one_rule_for_at_least_one_match::at_least_one_rule_for_at_least_one_match;
use log::{debug, error, info, trace};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
