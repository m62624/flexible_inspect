use super::*;
impl Subrules {
    pub fn get_default_r_vec(&self) -> &Vec<Rule> {
        &self.default_r_vec
    }
    pub fn get_fancy_r_vec(&self) -> &Vec<Rule> {
        &self.fancy_r_vec
    }
}
