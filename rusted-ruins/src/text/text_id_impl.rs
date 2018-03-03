
use super::ToTextId;
    
use common::gamedata::site::DungeonKind;

impl ToTextId for DungeonKind {
    fn to_textid(&self) -> &'static str {
        match *self {
            DungeonKind::Cave => "!dungeon_kind.cave",
            DungeonKind::Ruin => "!dungeon_kind.ruin",
            DungeonKind::None => "!dungeon_kind.none",
        }
    }
}

