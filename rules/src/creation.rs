use super::Rule;
use common::gamedata::CreationRequiredTime;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Creation {
    pub required_time: HashMap<CreationRequiredTime, u16>,
    pub recipe_learning_level_margin: u32,
}

impl Rule for Creation {
    const NAME: &'static str = "creation";

    fn append(&mut self, other: Self) {
        *self = other;
    }
}
