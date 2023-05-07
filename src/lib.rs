//! `Pystval` - `Rust` библиотека для `Python`. Выполняет валидацию каких либо данных (строки) с помощью регулярных выражений
mod export_py;
mod global_const;
use global_const::*;
use pyo3::{prelude::*, types, ToPyObject};
// Регуярные выражения
//=============================
use fancy_regex;
use regex;
//=============================
use std::{collections::HashMap, str};
/// Структруа для хранения ошибок и регулярных выражений
#[pyclass]
#[derive(Debug, Clone)]
pub struct TemplateValidator {
    // Храним KEY: `regex` и VALUE: `ID`
    all_simple_rules: HashMap<String, usize>,
    all_hard_rules: HashMap<String, usize>,
    // Собираем все regex и *одним проходом* проверяем все регулярки
    selected_simple_rules: regex::RegexSet,
    // Храним KEY: `ID` и VALUE: `ClassError`
    python_classes: HashMap<usize, PyObject>,
}
