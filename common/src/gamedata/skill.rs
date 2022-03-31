#![allow(clippy::manual_non_exhaustive)]

use fnv::FnvHashMap;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, PartialEq, Eq, Debug, Error)]
#[error("invalid string {0}")]
pub struct KindParseError(pub(crate) String);

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

macro_rules! define_skill_kind {
    {
        basic_skills = $basic_skill_start_value:expr;
        {
            $($basic_skill:ident, $basic_skill_as_str:expr,)*
        }
        melee_weapon_kind = $melee_weapon_start_value:expr;
        {
            $($melee_weapon:ident, $melee_weapon_as_str:expr,)*
        }
        ranged_weapon_kind = $ranged_weapon_start_value:expr;
        {
            $($ranged_weapon:ident, $ranged_weapon_as_str:expr,)*
        }
        creation_kind = $creation_start_value:expr;
        {
            $($creation:ident, $creation_as_str:expr,)*
        }
        special_skill_kind = $special_skill_start_value:expr;
        {
            $($special_skill:ident, $special_skill_as_str:expr,)*
        }
    } => {
        #[repr(u16)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
        pub enum SkillKind {
            #[doc(hidden)]
            _DummyBasicSkill = $basic_skill_start_value,
            $(
                $basic_skill,
            )*
            #[doc(hidden)]
            _DummyMeleeWeaponSkill = $melee_weapon_start_value,
            $(
                $melee_weapon,
            )*
            #[doc(hidden)]
            _DummyRangedWeaponSkill = $ranged_weapon_start_value,
            $(
                $ranged_weapon,
            )*
            #[doc(hidden)]
            _DummyCreationSkill = $creation_start_value,
            $(
                $creation,
            )*
            #[doc(hidden)]
            _DummySpecialSkill = $special_skill_start_value,
            $(
                $special_skill,
            )*
        }

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
        pub enum WeaponKind {
            $(
                $melee_weapon,
            )*
            #[doc(hidden)]
            _DummyWeapon = 0x7F,
            $(
                $ranged_weapon,
            )*
        }

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
        pub enum CreationKind {
            $(
                $creation,
            )*
        }

        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
        pub enum SpecialSkillKind {
            $(
                $special_skill,
            )*
        }

        impl SkillKind {
            pub fn weapon(self) -> Option<WeaponKind> {
                match self {
                    $(
                        SkillKind::$melee_weapon => Some(WeaponKind::$melee_weapon),
                    )*
                    $(
                        SkillKind::$ranged_weapon => Some(WeaponKind::$ranged_weapon),
                    )*
                    _ => None,
                }
            }

            pub fn creation(self) -> Option<CreationKind> {
                match self {
                    $(
                        SkillKind::$creation => Some(CreationKind::$creation),
                    )*
                    _ => None,
                }
            }

            pub fn textid(self) -> &'static str {
                if let Some(weapon_kind) = self.weapon() {
                    return weapon_kind.textid();
                }
                if let Some(creation_kind) = self.creation() {
                    return creation_kind.textid();
                }
                match self {
                    $(
                        SkillKind::$basic_skill => concat!("skill_kind-", $basic_skill_as_str),
                    )*
                    $(
                        SkillKind::$special_skill => concat!("skill_kind-", $special_skill_as_str),
                    )*
                    _ => unreachable!(),
                }
            }
        }

        impl FromStr for SkillKind {
            type Err = KindParseError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $(
                        $basic_skill_as_str => SkillKind::$basic_skill,
                    )*
                    $(
                        $melee_weapon_as_str => SkillKind::$melee_weapon,
                    )*
                    $(
                        $ranged_weapon_as_str => SkillKind::$ranged_weapon,
                    )*
                    $(
                        $creation_as_str => SkillKind::$creation,
                    )*
                    $(
                        $special_skill_as_str => SkillKind::$special_skill,
                    )*
                    _ => {
                        return Err(KindParseError(s.to_owned()));
                    }
                })
            }
        }

        impl FromStr for WeaponKind {
            type Err = KindParseError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $(
                        $melee_weapon_as_str => WeaponKind::$melee_weapon,
                    )*
                    $(
                        $ranged_weapon_as_str => WeaponKind::$ranged_weapon,
                    )*
                    _ => {
                        return Err(KindParseError(s.to_owned()));
                    }
                })
            }
        }

        impl From<WeaponKind> for SkillKind {
            fn from(weapon_kind: WeaponKind) -> Self {
                match weapon_kind {
                    $(
                        WeaponKind::$melee_weapon => SkillKind::$melee_weapon,
                    )*
                    $(
                        WeaponKind::$ranged_weapon => SkillKind::$ranged_weapon,
                    )*
                    _ => unreachable!(),
                }
            }
        }

        impl From<CreationKind> for SkillKind {
            fn from(creation_kind: CreationKind) -> Self {
                match creation_kind {
                    $(
                        CreationKind::$creation => SkillKind::$creation,
                    )*
                }
            }
        }

        impl From<SpecialSkillKind> for SkillKind {
            fn from(special_skill_kind: SpecialSkillKind) -> Self {
                match special_skill_kind {
                    $(
                        SpecialSkillKind::$special_skill => SkillKind::$special_skill,
                    )*
                }
            }
        }

        impl WeaponKind {
            pub fn is_melee(self) -> bool {
                self < WeaponKind::_DummyWeapon
            }

            pub fn is_ranged(self) -> bool {
                !self.is_melee()
            }

            pub fn textid(self) -> &'static str {
                match self {
                    $(
                        WeaponKind::$melee_weapon => concat!("weapon_kind-", $melee_weapon_as_str),
                    )*
                    $(
                        WeaponKind::$ranged_weapon => concat!("weapon_kind-", $ranged_weapon_as_str),
                    )*
                    WeaponKind::_DummyWeapon => unreachable!(),
                }
            }

            pub const ALL: &'static [WeaponKind] = &[
                $(
                    WeaponKind::$melee_weapon,
                )*
                $(
                    WeaponKind::$ranged_weapon,
                )*
            ];
        }

        impl std::fmt::Display for WeaponKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match self {
                    $(
                        WeaponKind::$melee_weapon => $melee_weapon_as_str,
                    )*
                    $(
                        WeaponKind::$ranged_weapon => $ranged_weapon_as_str,
                    )*
                    WeaponKind::_DummyWeapon => unreachable!(),
                };
                write!(f, "{}", s)
            }
        }

        impl CreationKind {
            pub fn textid(self) -> &'static str {
                match self {
                    $(
                        CreationKind::$creation => concat!("creation_kind-", $creation_as_str),
                    )*
                }
            }

            pub const ALL: &'static [CreationKind] = &[
                $(
                    CreationKind::$creation,
                )*
            ];
        }

        impl SpecialSkillKind {
            pub fn textid(self) -> &'static str {
                match self {
                    $(
                        SpecialSkillKind::$special_skill => concat!("special_skill_kind-", $special_skill_as_str),
                    )*
                }
            }
        }
    }
}

define_skill_kind! {
    basic_skills = 0;
    {
        BareHands, "bare_hands",
        Carrying, "carrying",
        Defence, "defence",
        Endurance, "endurance",
        Evasion, "evasion",
        Conceal, "conceal",
        Detection, "detection",
        Magic, "magic",
        MagicDevice, "magic_device",
        Mining, "mining",
        Plants, "plants",
        Animals, "animals",
        Leadership, "leadership",
    }
    melee_weapon_kind = 0x0400;
    {
        Sword, "sword",
        Spear, "spear",
        Axe, "axe",
        Blunt, "blunt",
        Whip, "whip",
    }
    ranged_weapon_kind = 0x0800;
    {
        Throwing, "throwing",
        Bow, "bow",
        Crossbow, "crossbow",
        Firearm, "firearm",
    }
    creation_kind = 0x1000;
    {
        Art, "art",
        Construction, "construction",
        Cooking, "cooking",
        Craft, "craft",
        Pharmacy, "pharmacy",
        Smith, "smith",
    }
    special_skill_kind = 0x1200;
    {
        Heat, "heat",
        Cold, "cold",
        Shock, "shock",
        Spirit, "spirit",
    }
}
