use super::Rule;
use common::gamedata::CharaModifier;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CharaTraits(HashMap<String, CharaTrait>);

impl CharaTraits {
    pub fn get(&self, id: &str) -> &CharaTrait {
        &self.0[id]
    }
}

impl Rule for CharaTraits {
    const NAME: &'static str = "traits";

    fn append(&mut self, other: Self) {
        for (k, v) in other.0.into_iter() {
            self.0.insert(k, v);
        }
    }
}

/// Rules for character parameter calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct CharaTrait {
    #[serde(default)]
    pub cost: i32,
    pub properties: Vec<CharaModifier>,
}
