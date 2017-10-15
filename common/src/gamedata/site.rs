
use std::collections::HashMap;
use super::map::{Map, MapId};

/// Site represents a dungeon, town, or other facility
/// It is consist of one or multiple maps
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    pub name: String,
    map: Vec<Map>,
    max_floor: u32,
}

impl Site {
    pub fn new(name: &str, max_floor: u32) -> Site {
        Site {
            name: name.to_owned(),
            map: Vec::new(),
            max_floor,
        }
    }
    
    pub fn get_map(&self, floor: u32) -> &Map {
        &self.map[floor as usize]
    }

    pub fn get_map_mut(&mut self, floor: u32) -> &mut Map {
        &mut self.map[floor as usize]
    }

    pub(crate) fn add_map(&mut self, map: Map) -> u32 {
        assert!(self.map.len() as u32 + 1 <= self.max_floor);
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

#[derive(Serialize, Deserialize)]
pub struct SiteHolder(pub(crate) HashMap<SiteId, Site>);

impl SiteHolder {
    pub(crate) fn new() -> SiteHolder {
        SiteHolder(HashMap::new())
    }
    
    pub fn get(&self, sid: SiteId) -> &Site {
        self.0.get(&sid).expect(&super::unknown_id_err(sid))
    }

    pub fn get_mut(&mut self, sid: SiteId) -> &mut Site {
        self.0.get_mut(&sid).expect(&super::unknown_id_err(sid))
    }

    pub fn get_map(&self, mid: MapId) -> &Map {
        self.get(mid.sid).get_map(mid.floor)
    }

    pub fn get_map_mut(&mut self, mid: MapId) -> &mut Map {
        self.get_mut(mid.sid).get_map_mut(mid.floor)
    }
}
