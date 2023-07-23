pub mod rules;

const ERR_OPTION: &str = " The body of `Rule` is missing, maybe you used modifiers, they borrow `Rule`, modifiers modify it and return the already modified version";

pub const DEFAULT_CAPTURE: &str = "main_capture";
