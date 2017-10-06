
use super::map::Map;

/// Site represents a dungeon, town, or other facility
/// It is consist of one or multiple maps
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    pub id: SiteId,
    pub name: String,
    map: Vec<Map>,
}

impl Site {
    pub fn get_map(&self, floor: u32) -> &Map {
        &self.map[floor as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SiteKind {
    /// Auto generated dungeon
    AutoGenDungeon,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum SiteId {
    /// Auto generated dungeon
    AutoGenDungeon { n: u32 },
}

