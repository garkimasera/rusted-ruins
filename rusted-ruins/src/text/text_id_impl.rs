
use super::ToTextId;
    
use common::gamedata::site::DungeonKind;
use common::gamedata::chara::CharaStatus;

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
            CharaStatus::Asleep { .. } => "!chara_status.asleep",
            CharaStatus::Poisoned      => "!chara_status.poisoned",
        }
    }
}
