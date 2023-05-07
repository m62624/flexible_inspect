//                          CLASS FROM PYTHON
//===================================================================
// имя атрибута, где хранится само сообщение и **extra переменные из класса Python
pub const MESSAGE_WITH_EXTRA_FROM_CLASS_PY: &'static str = "template";
// имя атрибута, где хранится регулярные выражения из класса Python
pub const RULES_FROM_CLASS_PY: &'static str = "rules";
//===================================================================

//                          STRUCTURE FROM RUST
//===================================================================
// имя атрибута, где хранится классы из Python на основе которых будут созданы ошибки
pub const CLASS_FOR_ERROR_PY: &'static str = "original_class";
// имя атрибута, где хранится **extra для заполнения информаций ошибки
pub const EXTRA_FOR_ERROR_PY: &'static str = "extra";
// имя атрибута, где хранится регулярные выражения для обработки данных
pub const REGEX_FOR_CHECK_DATA: &'static str = "rgxs";
// имя атрибута, где хранится коллекция для обработки в rust
pub const DATA_FOR_RUST: &'static str = "inner";
//===================================================================
pub const REGEX_FIND_ERROR: &'static str = "";
