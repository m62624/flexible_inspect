use super::*;
use std::collections::hash_map;
use std::hash::Hash;
use std::hash::Hasher;

mod fn_new {
    use super::*;

    /// Проверяем, что хеши одинаковые при одинаковых параметрах
    #[test]
    fn new_hash_t_0() {
        let mut hasher_1 = hash_map::DefaultHasher::new();
        let rule_1 = Rule::new(r"\s", MatchRequirement::MustBeFound);
        rule_1.hash(&mut hasher_1);
        let hasher_1_r = hasher_1.finish();

        let mut hasher_2 = hash_map::DefaultHasher::new();
        let rule_2 = Rule::new(r"\s", MatchRequirement::MustBeFound);
        rule_2.hash(&mut hasher_2);
        let hasher_2_r = hasher_2.finish();
        assert_eq!(hasher_1_r, hasher_2_r);
    }

    /// Проверяем, что хеши разные при разных параметрах
    #[test]
    fn new_hash_t_1() {
        let mut hasher_1 = hash_map::DefaultHasher::new();
        let rule_1 = Rule::new(r"\w", MatchRequirement::MustBeFound);
        rule_1.hash(&mut hasher_1);
        let hasher_1_r = hasher_1.finish();

        let mut hasher_2 = hash_map::DefaultHasher::new();
        let rule_2 = Rule::new(r"\s", MatchRequirement::MustBeFound);
        rule_2.hash(&mut hasher_2);
        let hasher_2_r = hasher_2.finish();
        assert_ne!(hasher_1_r, hasher_2_r);
    }

    #[test]
    fn new_hash_t_2() {
        let mut hasher_1 = hash_map::DefaultHasher::new();
        let rule_1 = Rule::new(r"\s", MatchRequirement::MustBeFound);
        rule_1.hash(&mut hasher_1);
        let hasher_1_r = hasher_1.finish();

        let mut hasher_2 = hash_map::DefaultHasher::new();
        let rule_2 = Rule::new(r"\s", MatchRequirement::MustBeFound);
        rule_2.hash(&mut hasher_2);
        let hasher_2_r = hasher_2.finish();
        assert_eq!(hasher_1_r, hasher_2_r);
    }
}
