
use fnv::FnvHashMap;
use super::item::WeaponKind;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum SkillKind {
    Defence,
    BareHands,
    Weapon(WeaponKind),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillList {
    pub skills: FnvHashMap<SkillKind, u16>,
    pub exp: Option<FnvHashMap<SkillKind, u16>>
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
    pub fn get(&self, kind: SkillKind) -> u16 {
        if let Some(skill) = self.skills.get(&kind) {
            *skill
        } else {
            0
        }
    }
}

/// Used to adjust exp of skills when getting exp
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Serialize, Deserialize)]
pub struct BaseLevel {
    level: u16,
    exp: u16,
}

impl BaseLevel {
    pub fn level(&self) -> u16 {
        self.level
    }

    pub fn add_exp(&mut self, exp: u16) {
        use basic::BASE_LEVEL_EXP_LVUP;

        if self.exp + exp <= BASE_LEVEL_EXP_LVUP {
            self.level += 1;
            self.exp = 0;
        } else {
            self.exp += exp;
        }
    }
}

