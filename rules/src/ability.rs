use super::Rule;
use common::gamedata::*;
use std::collections::{hash_map::Iter, HashMap};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(transparent)]
pub struct Abilities(HashMap<AbilityId, Ability>);

impl Abilities {
    pub fn iter(&self) -> Iter<'_, AbilityId, Ability> {
        self.0.iter()
    }
}

impl Rule for Abilities {
    const NAME: &'static str = "abilities";

    fn append(&mut self, other: Self) {
        for (k, v) in other.0.into_iter() {
            self.0.insert(k, v);
        }
    }
}

impl Abilities {
    pub fn get(&self, id: &AbilityId) -> Option<&Ability> {
        self.0.get(id)
    }
}
