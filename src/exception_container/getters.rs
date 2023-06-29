use super::*;

impl ExceptionContainer {
    pub fn get_py_class(&self) -> &PyObject {
        &self.py_class
    }
    pub fn get_fancy_roots_vec(&self) -> &Option<Vec<Rule>> {
        &self.fancy_root_vec
    }
    pub fn get_default_roots_vec(&self) -> &Option<Vec<Rule>> {
        &self.default_roots_vec
    }
    fn get_roots_set(&self) -> &Option<regex::RegexSet> {
        &self.default_roots_set
    }
}
