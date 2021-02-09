use common::basic::BonusLevel;
use common::gamedata::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Classes(HashMap<CharaClass, Class>);

impl Classes {
    pub fn get(&self, chara_class: CharaClass) -> &Class {
        self.0
            .get(&chara_class)
            .unwrap_or_else(|| &self.0[&CharaClass::default()])
    }
}

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct Class {
    /// Attribute revisions by class
    pub revision: CharaAttrRevision,
    /// Skill bonus
    pub skill_bonus: HashMap<SkillKind, BonusLevel>,
}
