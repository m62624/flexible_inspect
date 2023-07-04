use super::*;

impl Cartridge {
    pub fn get_py_class(&self) -> &PyObject {
        &self.py_class
    }
}

impl CartridgeWrapper {
    pub fn get_cartridge(&self) -> &Cartridge {
        &self.0
    }
}
