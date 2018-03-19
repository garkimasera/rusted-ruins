
use super::map::Map;
use super::region::RegionId;
use super::town::Town;

/// Site represents a dungeon, town, or other facility
/// It is consist of one or multiple maps
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    pub name: Option<String>,
    map: Vec<Map>,
    /// The maximum nubmer of floor
    max_floor: u32,
    /// Site kind specific data
    pub content: SiteContent,
}

/// Site kind specific data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SiteContent {
    /// Generated dungeons that will be created on region map repeatedly
    /// Player will explore them
    AutoGenDungeon {
        /// It is used for map generation, enemy race weighting, etc.
        dungeon_kind: DungeonKind,
    },
    /// Town consists of residents and shops, etc.
    Town {
        /// Detail data for this town
        town: Box<Town>,
    },
    /// This does not include specific data, but character and other elements can be placed its map
    Other,
}

impl Site {
    pub fn new(max_floor: u32) -> Site {
        Site {
            name: None,
            map: Vec::new(),
            max_floor,
            content: SiteContent::Other,
        }
    }
    
    pub fn get_map(&self, floor: u32) -> &Map {
        &self.map[floor as usize]
    }

    pub fn get_map_mut(&mut self, floor: u32) -> &mut Map {
        &mut self.map[floor as usize]
    }

    pub fn get_map_checked(&self, floor: u32) -> Option<&Map> {
        self.map.get(floor as usize)
    }

    pub fn get_map_mut_checked(&mut self, floor: u32) -> Option<&mut Map> {
        self.map.get_mut(floor as usize)
    }

    pub(crate) fn add_map(&mut self, map: Map) -> u32 {
        assert!(self.map.len() as u32 + 1 <= self.max_floor);
        self.map.push(map);
        self.map.len() as u32 - 1
    }
    
    pub fn is_underground(&self) -> bool {
        match self.content {
            SiteContent::AutoGenDungeon { dungeon_kind, .. } => {
                dungeon_kind.is_underground()
            }
            _ => false,
        }
    }

    pub fn floor_num(&self) -> u32 {
        self.map.len() as u32
    }

    pub fn max_floor(&self) -> u32 {
        self.max_floor
    }
}

impl SiteContent {
    pub fn kind(&self) -> SiteKind {
        match self {
            &SiteContent::AutoGenDungeon { .. } => { SiteKind::AutoGenDungeon },
            &SiteContent::Town { .. } => { SiteKind::Town },
            &SiteContent::Other { .. } => { SiteKind::Other },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(rename_all="snake_case")]
pub enum SiteKind {
    AutoGenDungeon, Town, Other
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct SiteId {
    pub rid: RegionId,
    pub kind: SiteKind,
    pub n: u32,
}

impl Default for SiteId {
    fn default() -> SiteId {
        SiteId {
            rid: RegionId::default(),
            kind: SiteKind::Other,
            n: 0
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum DungeonKind {
    None, Cave, Ruin,
}

impl DungeonKind {
    /// If the dungeon is in underground, it returns true.
    /// Player can go to deeper floors using downstairs tiles, and the exit is upstairs tile.
    /// If not, upstairs tile is used to go to deeper floor lile towers.
    pub fn is_underground(&self) -> bool {
        match *self {
            DungeonKind::Cave => true,
            _ => false,
        }
    }
}

