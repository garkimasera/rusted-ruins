use super::Rule;
use common::gamedata::*;

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct CharaGen {
    /// List of skills all character must have
    pub common_skills: Vec<SkillKind>,
    /// Equipment slots all character must have
    pub equip_slots: Vec<EquipSlotKind>,
}

impl Rule for CharaGen {
    const NAME: &'static str = "chara_gen";
}
