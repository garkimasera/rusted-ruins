use common::basic::BonusLevel;
use common::gamedata::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
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
    pub skill_bonus: HashMap<String, BonusLevel>,
}

impl Class {
    pub fn skill_bonus(&self, skill_kind: SkillKind) -> BonusLevel {
        for (skill_name, bonus) in &self.skill_bonus {
            let k: SkillKind = if let Ok(k) = skill_name.parse() {
                k
            } else {
                error!("unknown skill {}", skill_name);
                continue;
            };
            if k == skill_kind {
                return *bonus;
            }
        }
        BonusLevel::None
    }
}
