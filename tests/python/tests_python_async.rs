use pyo3::PyResult;
use pystval::prelude::*;

#[pyo3_asyncio::async_std::main]
async fn main() -> pyo3::PyResult<()> {
    pyo3_asyncio::testing::main().await
}

#[pyo3_asyncio::async_std::test]
async fn fn_async_validate_0() -> PyResult<()> {
    let cartridge = Cartridge::new(
        101,
        "message",
        [Rule::new("ABC", MatchRequirement::MustNotBeFound)],
    );
    let validator_text = TemplateValidator::new([cartridge]);
    let error_py = PystvalError::new(101, "message".into());
    if let Err(error) = validator_text.async_validate("ABC").await {
        assert_eq!(error[0], error_py);
    } else {
        panic!("expected error from validator");
    }
    Ok(())
}

#[pyo3_asyncio::async_std::test]
async fn fn_async_validate_1() -> PyResult<()> {
    let cartridge_1 = Cartridge::new(
        0,
        "message_0",
        [Rule::new("ABC", MatchRequirement::MustNotBeFound)],
    );
    let cartridge_2 = Cartridge::new(
        1,
        "message_1",
        [Rule::new(r"\d+", MatchRequirement::MustNotBeFound)],
    );
    let cartridge_3 = Cartridge::new(
        2,
        "message_2",
        [Rule::new(r"\w+", MatchRequirement::MustBeFound)],
    );

    let validator_text = TemplateValidator::new([cartridge_1, cartridge_2, cartridge_3]);

    let error_py_0 = PystvalError::new(0, "message_0".into());
    let error_py_1 = PystvalError::new(1, "message_1".into());
    if let Err(error) = validator_text.async_validate("ABC 123").await {
        assert_eq!(error[0], error_py_0);
        assert_eq!(error[1], error_py_1);
    } else {
        panic!("expected error from validator");
    }
    Ok(())
}
