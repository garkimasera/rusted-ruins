
/// This is helper trait for some data objects that need to be printed in game.
/// Returned text id is translated to appropriate words in text module.
pub trait ToTextId {
    fn to_textid(&self) -> &'static str;
}

use gamedata::site::DungeonKind;
impl ToTextId for DungeonKind {
    fn to_textid(&self) -> &'static str {
        match *self {
            DungeonKind::Cave => "!dungeon_kind.cave",
            DungeonKind::Ruin => "!dungeon_kind.ruin",
            DungeonKind::None => "!dungeon_kind.none",
        }
    }
}

