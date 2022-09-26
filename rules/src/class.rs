use crate::Rule;
use common::basic::BonusLevel;
use common::gamedata::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Classes(HashMap<CharaClass, Class>);

impl Rule for Classes {
    const NAME: &'static str = "classes";

    fn append(&mut self, other: Self) {
        for (k, v) in other.0.into_iter() {
            self.0.insert(k, v);
        }
    }
}

impl Classes {
    pub fn get(&self, chara_class: CharaClass) -> &Class {
        self.0
            .get(&chara_class)
            .unwrap_or_else(|| &self.0[&CharaClass::default()])
    }
}

/// Rules for character generation
#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    /// Attribute revisions by class
    pub attr: CharaAttrDiff,
    /// Skill bonus
    pub skill_bonus: HashMap<SkillKind, BonusLevel>,
    /// Item generation for equipment slot with bonus for gen level and quality
    #[serde(default)]
    pub equips: Vec<EquipGen>,
}
