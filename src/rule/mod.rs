use super::*;
//=========================
/// Создает regex паттерны для простых правил на основе полученного текста
mod generator;
/// Модуль для получения содержимого `Rule`
mod getters;
/// Модуль для инициализации `Rule`
mod init;
/// Модуль для модификации `Rule`
mod modifiers;
/// Модуль для запуска `Rule`
mod runner;
/// Типажи, необходимо для проверок `Eq` (contains)
mod traits;
//==========================
/// Модуль для проверки шага по правилу
pub mod next;
/// Модуль для разбиения правил на простые и сложные
pub mod slice;

// `Rule` - класс для хранения правила. При создания правила, автоматический
// определеяется тип правила ( `regex default ` | `regex fancy` )
// Идет проверка на валидность правила, + создается `RegexSet` для простых правил
//
// Каждый `Rule` может хранить подправила, сам же `Rule` может быть и корнем правил,
// и подправилом. Так же `Rule` может хранить `Counter` для ограничения количества совпадений
//
// ( на каждое правило создаются собственные модификаторы + `RegexSet` )\
// --> ExceptionContainer

/// `Rule` - a class for storing a rule with different modifiers
#[pyclass]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Rule {
    content: Option<TakeRuleForExtend>,
}

/// Содержимое `Rule`, необходим для модификаций Rule через std::mem::take\
/// --> Rule
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TakeRuleForExtend {
    pub str_with_type: RegexRaw,
    pub requirement: MatchRequirement,
    pub subrules: Option<Subrules>,
    pub counter: Option<Counter>,
    pub mod_match: ModeMatch,
}

/// `RegexRaw` - хранит тип правила и само правило, используется `Box<str>`,
/// так как мы не можем знать размер правила, + мы не можем использовать время жизни
/// так как используется в pyclass\
/// --> TakeRuleForExtend
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegexRaw {
    DefaultR(Box<str>),
    FancyR(Box<str>),
}

// `MatchRequirement` - модификатор правила, является неким флагом, который
// говорит, что по этому правилу должно быть найдено совпадение или же нет\
// --> TakeRuleForExtend

/// `MatchRequirement` - rule modifier, specifies whether a match should be found or not
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchRequirement {
    /// По правилу должно быть найдено совпадение
    MustBeFound,
    /// По правилу не должно быть найдено совпадение
    MustNotBeFound,
}

/// `Subrules` - хранит в себе простые и сложные правила, которые являются подправилами
/// для текущего правила, также используется как RootRules для классов\
/// --> TakeRuleForExtend\
/// --> Cartridge
#[derive(Debug, Clone)]
pub struct Subrules {
    pub simple_rules: Option<SimpleRules>,
    pub complex_rules: Option<Vec<Rule>>,
}

/// `Counter` - модификатор правила, является счетчиком совпадения
#[derive(Debug, Clone, Copy)]
pub enum Counter {
    /// Ровно столько раз, сколько указано X
    Only(usize),
    /// Большье или равно X
    MoreThan(usize),
    /// Меньше или равно X
    LessThan(usize),
}

#[pyclass]
#[derive(Debug, Clone)]
pub enum ModeMatch {
    /// Все подправила должны сработать успешно для всех совпадений
    AllRulesForAllMatches,
    /// Все подправила должны сработать успешно хотя бы для одного совпадения
    AllRulesForAtLeastOneMatch,
    /// Хотя бы одно правило должно сработать успешно для всех совпадений
    AtLeastOneRuleForAllMatches,
    /// Хотя бы одно правило должно сработать успешно хотя бы для одного совпадения
    AtLeastOneRuleForAtLeastOneMatch,
}

/// `SimpleRules` - хранит в себе простые правила + RegexSet

/*
   RegexSet позволяет избежать повторной компиляции регулярных выражений при выполнении нескольких проверок на одних и тех же шаблонах.
   Он также предлагает оптимизации для эффективного сравнения множества регулярных выражений на одной строке.

   Если у вас есть сотни или тысячи регулярных выражений для многократного сопоставления, то набор регулярных выражений может обеспечить огромный прирост производительности.
*/

/// --> Subrules
#[derive(Debug, Clone)]
pub struct SimpleRules {
    pub all_rules: Vec<Rule>,
    pub regex_set: regex::RegexSet,
}
