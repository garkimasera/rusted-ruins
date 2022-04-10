use crate::Rule;
use common::gamedata::*;
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Races(HashMap<String, Race>);

impl Races {
    pub fn get(&self, id: &str) -> &Race {
        self.get_id_race(id).1
    }

    pub fn get_id_race<'a>(&'a self, id: &str) -> (&'a str, &'a Race) {
        if let Some((id, race)) = self.0.get_key_value(id) {
            (id, race)
        } else {
            error!("tried to get unknown race \"{}\"", id);
            self.0
                .get_key_value("!")
                .map(|(id, race)| (id.as_str(), race))
                .unwrap()
        }
    }

    /// Get races as an iterator including base races
    pub fn iter<'a>(&'a self, id: &str) -> impl Iterator<Item = &'a Race> {
        self.get_races(id).into_iter().map(|(_, race)| race)
    }

    /// Get race ids as an iterator including base races
    pub fn iter_ids<'a>(&'a self, id: &str) -> impl Iterator<Item = &'a str> {
        self.get_races(id).into_iter().map(|(id, _)| id)
    }

    fn get_races<'a>(&'a self, id: &str) -> SmallVec<[(&'a str, &'a Race); 4]> {
        let (id, race) = self.get_id_race(id);
        if race.base_race.is_empty() {
            smallvec![(id, race)]
        } else {
            let mut v = self.get_races(&race.base_race);
            v.push((id, race));
            v
        }
    }
}

impl Rule for Races {
    const NAME: &'static str = "races";

    fn append(&mut self, other: Self) {
        for (k, v) in other.0.into_iter() {
            self.0.insert(k, v);
        }
    }
}

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct Race {
    #[serde(default)]
    pub base_race: String,
    /// Available Equipment slots
    #[serde(default)]
    pub equip_slots: Vec<EquipSlotKind>,
    /// Default element protection
    #[serde(default)]
    pub element_protection: ElementArray<ElementProtection>,
    /// Race traits.
    #[serde(default)]
    pub traits: Vec<CharaTrait>,
}
