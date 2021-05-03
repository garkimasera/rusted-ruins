use common::basic::BonusLevel;
use common::gamedata::*;
use common::item_selector::ItemSelector;
use serde_with::{serde_as, DisplayFromStr};
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
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Class {
    /// Attribute revisions by class
    pub revision: CharaAttrRevision,
    /// Skill bonus
    pub skill_bonus: HashMap<SkillKind, BonusLevel>,
    /// Item generation for equipment slot. The last number is level bonus.
    #[serde(default)]
    #[serde_as(as = "Vec<(_, DisplayFromStr, _)>")]
    pub equips: Vec<(EquipSlotKind, ItemSelector, u32)>,
}
