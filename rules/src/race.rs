use common::gamedata::*;
use std::collections::HashMap;

pub type Races = HashMap<String, Race>;

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct Race {
    /// Available Equipment slots
    pub equip_slots: Vec<EquipSlotKind>,
    /// Default element protection
    pub element_protection: ElementArray<ElementProtection>,
    /// Race traits.
    pub traits: Vec<CharaTrait>,
}
