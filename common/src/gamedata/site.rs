
use super::map::Map;

/// Site represents a dungeon, town, or other facility
/// It is consist of one or multiple maps
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    pub name: String,
    map: Vec<Map>,
}

impl Site {
    pub fn new(name: &str) -> Site {
        Site {
            name: name.to_owned(),
            map: Vec::new(),
        }
    }
    
    pub fn get_map(&self, floor: u32) -> &Map {
        &self.map[floor as usize]
    }

    pub fn get_map_mut(&mut self, floor: u32) -> &mut Map {
        &mut self.map[floor as usize]
    }

    pub(crate) fn add_map(&mut self, map: Map) -> u32 {
        self.map.push(map);
        self.map.len() as u32 - 1
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SiteKind {
    Start,
    /// Auto generated dungeon
    AutoGenDungeon,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum SiteId {
    Start,
    /// Auto generated dungeon
    AutoGenDungeon { n: u32 },
}

