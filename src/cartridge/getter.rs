use super::*;

impl Cartridge {
    /// Получаем ссылку на класс
    pub fn get_py_class(&self) -> &PyObject {
        &self.py_class
    }
}

impl CartridgeWrapper {
    /// Получаем ссылку на `Cartridge`
    pub fn get_cartridge(&self) -> &Cartridge {
        &self.0
    }
}
