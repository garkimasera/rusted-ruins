use common::gamedata::*;
use std::collections::HashMap;

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct CharaGen {
    /// List of skills all character must have
    pub common_skills: Vec<SkillKind>,
    /// Default equipment slots by race
    pub default_equip_slots: HashMap<Race, Vec<(EquipSlotKind, u8)>>,
}
