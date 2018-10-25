
use super::ToTextId;
    
use common::gamedata::*;

impl ToTextId for DungeonKind {
    fn to_textid(&self) -> &'static str {
        match *self {
            DungeonKind::Cave => "!dungeon_kind.cave",
            DungeonKind::Ruin => "!dungeon_kind.ruin",
            DungeonKind::None => "!dungeon_kind.none",
        }
    }
}

impl ToTextId for CharaStatus {
    fn to_textid(&self) -> &'static str {
        match *self {
            CharaStatus::Hungry        => "!chara_status.hungry",
            CharaStatus::Weak          => "!chara_status.weak",
            CharaStatus::Starving      => "!chara_status.starving",
            CharaStatus::Asleep { .. } => "!chara_status.asleep",
            CharaStatus::Poisoned      => "!chara_status.poisoned",
        }
    }
}

impl ToTextId for SkillKind {
   fn to_textid(&self) -> &'static str {
       match self {
           SkillKind::BareHands     => "!skill_kind.bare_hands",
           SkillKind::Defence       => "!skill_kind.defence",
           SkillKind::Weapon(weapon_kind) => {
               weapon_kind.to_textid()
           }
       }
   }
}

impl ToTextId for WeaponKind {
    fn to_textid(&self) -> &'static str {
        match self {
            WeaponKind::Axe      => "!weapon_kind.axe",
            WeaponKind::Bow      => "!weapon_kind.bow",
            WeaponKind::Crossbow => "!weapon_kind.cross_bow",
            WeaponKind::Gun      => "!weapon_kind.gun",
            WeaponKind::Spear    => "!weapon_kind.spear",
            WeaponKind::Sword    => "!weapon_kind.sword",
            WeaponKind::Whip     => "!weapon_kind.whip",
        }
    }
}
