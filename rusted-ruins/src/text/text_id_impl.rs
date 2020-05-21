use super::ToTextId;

use common::gamedata::*;

impl ToTextId for DungeonKind {
    fn to_textid(&self) -> &'static str {
        match *self {
            DungeonKind::Cave => "dungeon_kind-cave",
            DungeonKind::Ruin => "dungeon_kind-ruin",
            DungeonKind::None => "dungeon_kind-none",
        }
    }
}

impl ToTextId for CharaStatus {
    fn to_textid(&self) -> &'static str {
        match *self {
            CharaStatus::Hungry => "chara_status-hungry",
            CharaStatus::Weak => "chara_status-weak",
            CharaStatus::Starving => "chara_status-starving",
            CharaStatus::Burdened => "chara_status-burdened",
            CharaStatus::Stressed => "chara_status-stressed",
            CharaStatus::Strained => "chara_status-strained",
            CharaStatus::Overloaded => "chara_status-overloaded",
            CharaStatus::Asleep { .. } => "chara_status-asleep",
            CharaStatus::Poisoned => "chara_status-poisoned",
            CharaStatus::Creation { .. } => "chara_status-creation",
        }
    }
}

impl ToTextId for ItemKind {
    fn to_textid(&self) -> &'static str {
        use ItemKind::*;
        match self {
            Potion => "item_kind-potion",
            Food => "item_kind-food",
            MagicDevice => "item_kind-magic_device",
            Weapon(weapon_kind) => weapon_kind.to_textid(),
            Armor(armor_kind) => armor_kind.to_textid(),
            Tool => "item_kind-tool",
            Container => "item_kind-container",
            Special => "item_kind-special",
            Material => "item_kind-material",
            Object => "item_kind-object",
        }
    }
}

impl ToTextId for SkillKind {
    fn to_textid(&self) -> &'static str {
        use SkillKind::*;
        match self {
            Endurance => "skill_kind-endurance",
            Healing => "skill_kind-healing",
            BareHands => "skill_kind-bare_hands",
            Defence => "skill_kind-defence",
            Evasion => "skill_kind-evasion",
            Carrying => "skill_kind-carrying",
            MagicDevice => "skill_kind-magic_device",
            Weapon(weapon_kind) => weapon_kind.to_textid(),
        }
    }
}

impl ToTextId for WeaponKind {
    fn to_textid(&self) -> &'static str {
        use WeaponKind::*;
        match self {
            Axe => "weapon_kind-axe",
            Bow => "weapon_kind-bow",
            Crossbow => "weapon_kind-cross_bow",
            Firearm => "weapon_kind-firearm",
            Spear => "weapon_kind-spear",
            Sword => "weapon_kind-sword",
            Whip => "weapon_kind-whip",
        }
    }
}

impl ToTextId for ArmorKind {
    fn to_textid(&self) -> &'static str {
        use ArmorKind::*;
        match self {
            Body => "armor_kind-body",
            Shield => "armor_kind-shield",
        }
    }
}
