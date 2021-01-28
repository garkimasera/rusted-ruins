use super::defs::CreationKind;
use super::item::WeaponKind;
use fnv::FnvHashMap;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillKind {
    BareHands,
    Carrying,
    Defence,
    Endurance,
    Evasion,
    Healing,
    Throwing,
    MagicDevice,
    Mining,
    Weapon(WeaponKind),
    Creation(CreationKind),
}

#[derive(Clone, PartialEq, Eq, Debug, Error)]
#[error("invalid string {0}")]
pub struct SkillKindParseError(String);

impl FromStr for SkillKind {
    type Err = SkillKindParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "bare_hands" => SkillKind::BareHands,
            "carrying" => SkillKind::Carrying,
            "defence" => SkillKind::Defence,
            "endurance" => SkillKind::Endurance,
            "evasion" => SkillKind::Evasion,
            "healing" => SkillKind::Healing,
            "throwing" => SkillKind::Throwing,
            "magic_device" => SkillKind::MagicDevice,
            "mining" => SkillKind::Mining,
            "sword" => SkillKind::Weapon(WeaponKind::Sword),
            "spear" => SkillKind::Weapon(WeaponKind::Spear),
            "axe" => SkillKind::Weapon(WeaponKind::Axe),
            "whip" => SkillKind::Weapon(WeaponKind::Whip),
            "bow" => SkillKind::Weapon(WeaponKind::Bow),
            "cross_bow" => SkillKind::Weapon(WeaponKind::Crossbow),
            "fire_arm" => SkillKind::Weapon(WeaponKind::Firearm),
            "art" => SkillKind::Creation(CreationKind::Art),
            "construction" => SkillKind::Creation(CreationKind::Construction),
            "cooking" => SkillKind::Creation(CreationKind::Cooking),
            "craft" => SkillKind::Creation(CreationKind::Craft),
            "pharmacy" => SkillKind::Creation(CreationKind::Pharmacy),
            "smith" => SkillKind::Creation(CreationKind::Smith),
            _ => {
                return Err(SkillKindParseError(s.to_owned()));
            }
        })
    }
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
