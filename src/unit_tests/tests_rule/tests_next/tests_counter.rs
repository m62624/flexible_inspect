use super::*;

mod fn_counter_is_equal {
    use super::*;

    /// Проверка counter Only для next, всего совпадений - 3, должно быть 3
    #[test]
    fn counter_status_t_0() -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(3);
        let mut captures = CaptureData::find_captures(&rule, "lock lock lock");
        assert_eq!(rule.counter_status(&mut captures), NextStep::Go);
        Ok(())
    }

    /// Проверка counter Only для next, всего совпадений - 3, должно быть 4
    #[test]
    fn counter_status_t_1() -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_is_equal(4);
        let mut captures = CaptureData::find_captures(&rule, "lock lock lock");
        let err_data = captures.hashmap_for_error.clone();
        assert_eq!(
            rule.counter_status(&mut captures),
            NextStep::Error(Some(err_data))
        );
        Ok(())
    }
}

mod fn_counter_less_than {
    use super::*;

    /// Проверка counter LessThan для next, всего совпадений - 3, должно меньше 4 (включительно)
    #[test]
    fn counter_status_t_0() -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_less_than(4);
        let mut captures = CaptureData::find_captures(&rule, "lock lock lock");
        assert_eq!(rule.counter_status(&mut captures), NextStep::Go);
        Ok(())
    }

    /// Проверка counter LessThan для next, всего совпадений - 3, должно меньше 2 (включительно)
    #[test]
    fn counter_status_t_1() -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_less_than(2);
        let mut captures = CaptureData::find_captures(&rule, "lock lock lock");
        let err_data = captures.hashmap_for_error.clone();
        assert_eq!(
            rule.counter_status(&mut captures),
            NextStep::Error(Some(err_data))
        );
        Ok(())
    }
}

mod fn_couner_more_than {
    use super::*;

    /// Проверка counter MoreThan для next, всего совпадений - 3, должно быть больше 2 (включительно)
    #[test]
    fn counter_status_t_0() -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_more_than(2);
        let mut captures = CaptureData::find_captures(&rule, "lock lock lock");
        assert_eq!(rule.counter_status(&mut captures), NextStep::Go);
        Ok(())
    }

    /// Проверка counter MoreThan для next, всего совпадений - 3, должно быть больше 4 (включительно)
    #[test]
    fn counter_status_t_1() -> PyResult<()> {
        let rule = Rule::spawn(r"(\w+)", MatchRequirement::MustBeFound)?.counter_more_than(4);
        let mut captures = CaptureData::find_captures(&rule, "lock lock lock");
        let err_data = captures.hashmap_for_error.clone();
        assert_eq!(
            rule.counter_status(&mut captures),
            NextStep::Error(Some(err_data))
        );
        Ok(())
    }
}
