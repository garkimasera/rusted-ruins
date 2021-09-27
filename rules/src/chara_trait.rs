use super::Rule;
use common::gamedata::Property;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CharaTraits(HashMap<String, CharaTrait>);

impl Rule for CharaTraits {
    const NAME: &'static str = "chara_traits";

    fn append(&mut self, other: Self) {
        for (k, v) in other.0.into_iter() {
            self.0.insert(k, v);
        }
    }
}

/// Rules for character parameter calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct CharaTrait {
    cost: i32,
    properties: Vec<Property>,
}
