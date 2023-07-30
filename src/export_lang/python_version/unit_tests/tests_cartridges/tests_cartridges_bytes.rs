
use super::*;

#[test]
fn fn_new_t_0() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let rules_bytes = PyList::new(
            py,
            [PyRuleBytes::new(
                r"\d".into(),
                crate::export_lang::python_version::rules::PyMatchRequirement::MustBeFound,
            )
            .into_py(py)],
        );
        PyCartridgeBytes::new(py, 1, "f byte".into(), rules_bytes.into_py(py)).unwrap();
    })
}

#[test]
#[should_panic]
fn fn_new_t_1() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let rules_bytes = PyList::new(py, [FakeObject.into_py(py)]);
        PyCartridgeBytes::new(py, 1, "f byte".into(), rules_bytes.into_py(py)).unwrap();
    })
}

/// Провеярем matching mode
#[test]
fn fn_new_t_2() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let rust_rules_bytes = [PyRuleBytes::new(
            r"\d+".into(),
            crate::export_lang::python_version::rules::PyMatchRequirement::MustBeFound,
        )];
        let rules_bytes =
            PyList::new(py, rust_rules_bytes.into_iter().map(|x| x.into_py(py))).into_py(py);
        let mut cartridge_byte =
            PyCartridgeBytes::new(py, 1, "f byte".into(), rules_bytes).unwrap();
        assert_eq!(
            cartridge_byte.to_rust().root_rule.get_mode_match(),
            &ModeMatch::AllRulesForAllMatches
        );
    })
}

/// Провеярем matching mode
#[test]
fn fn_new_t_3() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let rust_rules_bytes = [PyRuleBytes::new(
            r"\d+".into(),
            crate::export_lang::python_version::rules::PyMatchRequirement::MustBeFound,
        )];
        let rules_bytes =
            PyList::new(py, rust_rules_bytes.into_iter().map(|x| x.into_py(py))).into_py(py);
        let mut cartridge_byte = PyCartridgeBytes::new(py, 1, "f byte".into(), rules_bytes)
            .unwrap()
            .mode_all_rules_for_at_least_one_match();
        assert_eq!(
            cartridge_byte.to_rust().root_rule.get_mode_match(),
            &ModeMatch::AllRulesForAtLeastOneMatch
        );
    })
}

/// Провеярем matching mode
#[test]
fn fn_new_t_4() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let rust_rules_bytes = [PyRuleBytes::new(
            r"\d+".into(),
            crate::export_lang::python_version::rules::PyMatchRequirement::MustBeFound,
        )];
        let rules_bytes =
            PyList::new(py, rust_rules_bytes.into_iter().map(|x| x.into_py(py))).into_py(py);
        let mut cartridge_byte = PyCartridgeBytes::new(py, 1, "f byte".into(), rules_bytes)
            .unwrap()
            .mode_at_least_one_rule_for_all_matches();
        assert_eq!(
            cartridge_byte.to_rust().root_rule.get_mode_match(),
            &ModeMatch::AtLeastOneRuleForAllMatches
        );
    })
}

/// Провеярем matching mode
#[test]
fn fn_new_t_5() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let rust_rules_bytes = [PyRuleBytes::new(
            r"\d+".into(),
            crate::export_lang::python_version::rules::PyMatchRequirement::MustBeFound,
        )];
        let rules_bytes =
            PyList::new(py, rust_rules_bytes.into_iter().map(|x| x.into_py(py))).into_py(py);
        let mut cartridge_byte = PyCartridgeBytes::new(py, 1, "f byte".into(), rules_bytes)
            .unwrap()
            .mode_at_least_one_rule_for_at_least_one_match();
        assert_eq!(
            cartridge_byte.to_rust().root_rule.get_mode_match(),
            &ModeMatch::AtLeastOneRuleForAtLeastOneMatch
        );
    })
}
