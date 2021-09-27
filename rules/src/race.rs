use crate::Rule;
use common::gamedata::*;
use std::collections::HashMap;

pub type Races = HashMap<String, Race>;

impl Rule for Races {
    const NAME: &'static str = "races";

    fn append(&mut self, other: Self) {
        for (k, v) in other.into_iter() {
            self.insert(k, v);
        }
    }
}

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct Race {
    /// Available Equipment slots
    pub equip_slots: Vec<EquipSlotKind>,
    /// Default element protection
    pub element_protection: ElementArray<ElementProtection>,
    /// Race traits.
    #[serde(default)]
    pub traits: Vec<CharaTrait>,
}
