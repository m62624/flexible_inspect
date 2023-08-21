pub mod cartridges_bytes;
pub mod cartridges_str;
use super::*;

const ERR_OPTION: &str = "\nThe body of `Cartridge` is missing (inside Cartridge is the value None), you may have used modifiers separately from initializations, they take the value (ownership) of `Cartridge` (std::mem::take) and return the already modified version (specify the modifier in the same place where you initialize `Cartridge|CartridgeBytes`).\n";
