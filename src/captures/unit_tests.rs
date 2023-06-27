#[cfg(test)]
use super::*;

#[cfg(test)]
/// Тестирование метода `find_captures` структуры `MultiCapture`
mod fn_find_captures {
    use super::*;

    /// Тестирование метода `find_captures`, раздел `Default Regex`
    #[test]
    fn find_captures_t_0() -> PyResult<()> {
        let mlcp = MultiCapture::find_captures(
            &Rule::spawn(r"111 222", rule::MatchRequirement::MustBeFound).unwrap(),
            "111 222 333",
        )?;
        dbg!(&mlcp);
        Ok(())
    }

    /// Тестирование метода `find_captures`, раздел `Fancy Regex`
    #[test]
    fn find_captures_t_1() -> PyResult<()> {
        let mlcp = MultiCapture::find_captures(
            &Rule::spawn(r"\d+\s(?=222)", rule::MatchRequirement::MustBeFound).unwrap(),
            "111 222 333",
        )?;
        dbg!(&mlcp);
        Ok(())
    }
}

/// Тестирование метода `is_some` структуры `MultiCapture`
#[cfg(test)]
mod fn_is_some {
    use super::*;

    /// Проверка совпадения, раздел `Default Regex` ( ( capturs > 0 ) = true )
    #[test]
    fn is_some_t_0() -> PyResult<()> {
        let mlcp = MultiCapture::find_captures(
            &Rule::spawn(r"111 222", rule::MatchRequirement::MustBeFound).unwrap(),
            "111 222 333",
        )?;
        assert!(mlcp.is_some());
        Ok(())
    }

    ///  Проверка совпадения, раздел `Fancy Regex` ( ( capturs > 0 ) = true )
    #[test]
    fn is_some_t_1() -> PyResult<()> {
        let mlcp = MultiCapture::find_captures(
            &Rule::spawn(r"\d+\s(?=222)", rule::MatchRequirement::MustBeFound).unwrap(),
            "111 222 333",
        )?;
        assert!(mlcp.is_some());
        Ok(())
    }

    ///  Проверка совпадения, раздел `Default Regex` ( ( capturs = 0 ) = false )
    #[test]
    fn is_some_t_2() -> PyResult<()> {
        let mlcp = MultiCapture::find_captures(
            &Rule::spawn(r"0", rule::MatchRequirement::MustBeFound).unwrap(),
            "1",
        )?;
        assert_eq!(mlcp.is_some(), false);
        Ok(())
    }

    ///  Проверка совпадения, раздел `Fancy Regex` ( ( capturs > 0 ) = false )
    #[test]
    fn is_some_t_3() -> PyResult<()> {
        let mlcp = MultiCapture::find_captures(
            &Rule::spawn(r"\d+\s(?=222)", rule::MatchRequirement::MustBeFound).unwrap(),
            "0",
        )?;
        assert_eq!(mlcp.is_some(), false);
        Ok(())
    }
}

/// Тестирование метода `to_str_vec` структуры `MultiCapture`
#[cfg(test)]
mod fn_to_str_vec {
    use super::*;

    /// Конвертация в `Vec<&str>`, раздел `Default Regex`
    #[test]
    fn to_str_vec_t_0() -> PyResult<()> {
        assert!(!MultiCapture::find_captures(
            &Rule::spawn(r"111 222", rule::MatchRequirement::MustBeFound).unwrap(),
            "111 222 333",
        )?
        .to_str_vec()
        .is_empty());
        Ok(())
    }

    /// Конвертация в `Vec<&str>`, раздел `Fancy Regex`
    #[test]
    fn to_str_vec_t_1() -> PyResult<()> {
        assert!(!MultiCapture::find_captures(
            &Rule::spawn(r"\d+\s(?=222)", rule::MatchRequirement::MustBeFound).unwrap(),
            "111 222 333",
        )?
        .to_str_vec()
        .is_empty());
        Ok(())
    }
}
