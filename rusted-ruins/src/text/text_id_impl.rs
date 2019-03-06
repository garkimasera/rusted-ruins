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
            CharaStatus::Hungry => "!chara_status.hungry",
            CharaStatus::Weak => "!chara_status.weak",
            CharaStatus::Starving => "!chara_status.starving",
            CharaStatus::Asleep { .. } => "!chara_status.asleep",
            CharaStatus::Poisoned => "!chara_status.poisoned",
        }
    }
}

impl ToTextId for ItemKind {
    fn to_textid(&self) -> &'static str {
        use ItemKind::*;
        match self {
            Potion => "!item_kind.potion",
            Herb => "!item_kind.herb",
            Food => "!item_kind.food",
            Weapon(weapon_kind) => weapon_kind.to_textid(),
            Armor(armor_kind) => armor_kind.to_textid(),
            Material => "!item_kind.material",
            Special => "!item_kind.special",
            Object => "!item_kind.object",
        }
    }
}

impl ToTextId for SkillKind {
    fn to_textid(&self) -> &'static str {
        use SkillKind::*;
        match self {
            Endurance => "!skill_kind.endurance",
            Healing => "!skill_kind.healing",
            MartialArts => "!skill_kind.martial_arts",
            Defence => "!skill_kind.defence",
            Evasion => "!skill_kind.evasion",
            Weapon(weapon_kind) => weapon_kind.to_textid(),
        }
    }
}

impl ToTextId for WeaponKind {
    fn to_textid(&self) -> &'static str {
        use WeaponKind::*;
        match self {
            Axe => "!weapon_kind.axe",
            Bow => "!weapon_kind.bow",
            Crossbow => "!weapon_kind.cross_bow",
            Gun => "!weapon_kind.gun",
            Spear => "!weapon_kind.spear",
            Sword => "!weapon_kind.sword",
            Whip => "!weapon_kind.whip",
        }
    }
}

impl ToTextId for ArmorKind {
    fn to_textid(&self) -> &'static str {
        use ArmorKind::*;
        match self {
            Body => "!armor_kind.body",
            Shield => "!armor_kind.shield",
        }
    }
}
