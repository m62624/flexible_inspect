use super::Rule;
mod getters;

#[derive(Debug, Clone)]
pub struct Subrules {
    default_r_vec: Vec<Rule>,
    fancy_r_vec: Vec<Rule>,
}

impl Subrules {
    pub fn new(default_r_vec: Vec<Rule>, fancy_r_vec: Vec<Rule>) -> Self {
        Self {
            default_r_vec,
            fancy_r_vec,
        }
    }
}
