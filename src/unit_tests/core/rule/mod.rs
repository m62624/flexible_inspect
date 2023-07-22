use super::*;

/// Проверка конструктора `Rule`
mod fn_new {
    use super::*;

    /// Создаем правило с помощью конструктора `Regex` (MatchRequirement::MustBeFound)
    #[test]
    fn new_t_0() {
        dbg!(Rule::new(r"(?-u)(?<cstr>[^\x00]+)\x00", MatchRequirement::MustBeFound));
    }
}
