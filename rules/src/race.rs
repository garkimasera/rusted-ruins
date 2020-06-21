use common::gamedata::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Races(HashMap<String, Race>);

impl Races {
    pub fn get(&self, race_name: &str) -> &Race {
        self.0.get(race_name).unwrap()
    }
}

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct Race {
    /// Available Equipment slots
    pub equip_slots: Vec<EquipSlotKind>,
    /// Default element protection
    pub element_protection: ElementArray<ElementProtection>,
}
