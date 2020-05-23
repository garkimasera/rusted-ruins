use common::gamedata::*;

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct CharaGen {
    /// List of skills all character must have
    pub common_skills: Vec<SkillKind>,
}
