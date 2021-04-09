use common::gamedata::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SpecialSkills(HashMap<SpecialSkillId, SpecialSkill>);

impl SpecialSkills {
    pub fn get(&self, id: &str) -> &SpecialSkill {
        self.0.get(id).unwrap_or_else(|| panic!())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecialSkillKind {
    Magic,
    Special,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialSkill {
    pub kind: SpecialSkillKind,
    pub effect: Effect,
}
