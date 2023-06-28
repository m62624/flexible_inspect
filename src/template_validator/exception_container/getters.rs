use super::*;
impl ExceptionContainer {
    pub fn get_class(&self) -> &PyObject {
        &self.py_class
    }
    pub fn get_default_rules(&self) -> &Vec<Rule> {
        &self.default_r
    }
    pub fn get_fancy_rules(&self) -> &Vec<Rule> {
        &self.fancy_r
    }
}
