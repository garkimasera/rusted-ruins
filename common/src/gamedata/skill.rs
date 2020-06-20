use super::item::WeaponKind;
use fnv::FnvHashMap;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillKind {
    Endurance,
    Healing,
    Defence,
    Evasion,
    Carrying,
    MagicDevice,
    BareHands,
    Weapon(WeaponKind),
}

#[derive(Clone, PartialEq, Eq, Debug, Error)]
#[error("invalid string {0}")]
pub struct SkillKindParseError(String);

impl FromStr for SkillKind {
    type Err = SkillKindParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "endurance" => SkillKind::Endurance,
            "healing" => SkillKind::Healing,
            "defence" => SkillKind::Defence,
            "evasion" => SkillKind::Evasion,
            "carrying" => SkillKind::Carrying,
            "magic_device" => SkillKind::MagicDevice,
            "bare_hands" => SkillKind::BareHands,
            "sword" => SkillKind::Weapon(WeaponKind::Sword),
            "spear" => SkillKind::Weapon(WeaponKind::Spear),
            "axe" => SkillKind::Weapon(WeaponKind::Axe),
            "whip" => SkillKind::Weapon(WeaponKind::Whip),
            "bow" => SkillKind::Weapon(WeaponKind::Bow),
            "cross_bow" => SkillKind::Weapon(WeaponKind::Crossbow),
            "fire_arm" => SkillKind::Weapon(WeaponKind::Firearm),
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
