/*
                                                                  tttt                                               lllllll
                                                               ttt:::t                                               l:::::l
                                                               t:::::t                                               l:::::l
                                                               t:::::t                                               l:::::l
ppppp   pppppppppyyyyyyy           yyyyyyy  ssssssssss   ttttttt:::::tttttttvvvvvvv           vvvvvvvaaaaaaaaaaaaa    l::::l
p::::ppp:::::::::py:::::y         y:::::y ss::::::::::s  t:::::::::::::::::t v:::::v         v:::::v a::::::::::::a   l::::l
p:::::::::::::::::py:::::y       y:::::yss:::::::::::::s t:::::::::::::::::t  v:::::v       v:::::v  aaaaaaaaa:::::a  l::::l
pp::::::ppppp::::::py:::::y     y:::::y s::::::ssss:::::stttttt:::::::tttttt   v:::::v     v:::::v            a::::a  l::::l
 p:::::p     p:::::p y:::::y   y:::::y   s:::::s  ssssss       t:::::t          v:::::v   v:::::v      aaaaaaa:::::a  l::::l
 p:::::p     p:::::p  y:::::y y:::::y      s::::::s            t:::::t           v:::::v v:::::v     aa::::::::::::a  l::::l
 p:::::p     p:::::p   y:::::y:::::y          s::::::s         t:::::t            v:::::v:::::v     a::::aaaa::::::a  l::::l
 p:::::p    p::::::p    y:::::::::y     ssssss   s:::::s       t:::::t    tttttt   v:::::::::v     a::::a    a:::::a  l::::l
 p:::::ppppp:::::::p     y:::::::y      s:::::ssss::::::s      t::::::tttt:::::t    v:::::::v      a::::a    a:::::a l::::::l
 p::::::::::::::::p       y:::::y       s::::::::::::::s       tt::::::::::::::t     v:::::v       a:::::aaaa::::::a l::::::l
 p::::::::::::::pp       y:::::y         s:::::::::::ss          tt:::::::::::tt      v:::v         a::::::::::aa:::al::::::l
 p::::::pppppppp        y:::::y           sssssssssss              ttttttttttt         vvv           aaaaaaaaaa  aaaallllllll
 p:::::p               y:::::y
 p:::::p              y:::::y
p:::::::p            y:::::y
p:::::::p           y:::::y
p:::::::p          yyyyyyy
ppppppppp

:D
*/
//! The Data validator is a universal tool for checking the correctness of data in string and byte formats. It allows you to determine whether the data conforms to certain rules and provides the ability to find errors and inconsistencies.\
//! The project aims to create a versatile and flexible tool for validating data in different formats, ensuring accuracy, reliability and usability.

// =====================================================================
mod cartridges;
mod error;
mod message;
mod rules;
mod template_validator;
// =====================================================================
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Once;
// =====================================================================
pub mod prelude {
    pub use crate::cartridges::{traits::CartridgeModifiers, Cartridge};
    pub use crate::error::traits::ValidationError;
    pub use crate::rules::MatchRequirement;
    pub use crate::rules::{rule_bytes::RuleBytes, rule_str::Rule, traits::RuleModifiers};
    pub use crate::template_validator::{TemplateValidator, ValidatorBase};
}
// =====================================================================
/// For one-time initialization to the logger
static INIT: Once = Once::new();
// =====================================================================
/// Initialization of the logger
#[cfg(not(tarpaulin_include))]
fn init_logger() {
    // env_logger is called only once
    INIT.call_once(|| {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("OFF")).init();
    });
}
