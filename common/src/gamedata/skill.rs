use super::item::WeaponKind;
use fnv::FnvHashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillKind {
    Endurance,
    Healing,
    Defence,
    Evasion,
    MartialArts,
    Weapon(WeaponKind),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillList {
    pub skills: FnvHashMap<SkillKind, u32>,
    pub exp: Option<FnvHashMap<SkillKind, u16>>,
}

impl Default for SkillList {
    fn default() -> SkillList {
        SkillList {
            skills: FnvHashMap::default(),
            exp: None,
        }
    }
}

impl SkillList {
    pub fn get(&self, kind: SkillKind) -> u32 {
        if let Some(skill) = self.skills.get(&kind) {
            *skill
        } else {
            0
        }
    }
}
