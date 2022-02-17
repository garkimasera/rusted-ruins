use super::map::{Map, MapId};
use super::region::RegionId;
use super::town::Town;
use super::{CharaId, Shop};
use crate::basic::ArrayStringId;
use crate::hashmap::HashMap;
use crate::sitegen::NpcGenId;
use filebox::FileBox;
use geom::Vec2d;

pub type BoxedMap = FileBox<Map>;

/// Site represents a dungeon, town, or other facility
/// It is consist of one or multiple maps
#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    pub name: Option<String>,
    /// id of SiteGenObject. None for auto generated sites.
    pub id: Option<String>,
    map: Vec<BoxedMap>,
    /// The maximum nubmer of floor
    max_floor: u32,
    /// Site kind specific data
    pub content: SiteContent,
    pub shops: HashMap<CharaId, Shop>,
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
    /// Player's base
    Player { kind: PlayerBaseKind },
    /// Temporary map
    Temp {
        return_map: MapId,
        return_pos: Vec2d,
        is_open_air: bool,
    },
    /// This does not include specific data, but character and other elements can be placed its map
    Other,
}

impl Site {
    pub fn new(max_floor: u32, id: Option<String>) -> Site {
        Site {
            name: None,
            id,
            map: Vec::new(),
            max_floor,
            content: SiteContent::Other,
            shops: HashMap::default(),
        }
    }

    pub fn get_map(&self, floor: u32) -> &Map {
        &self.map[floor as usize]
    }

    pub fn get_map_mut(&mut self, floor: u32) -> &mut Map {
        &mut self.map[floor as usize]
    }

    pub fn get_boxed_map_mut(&mut self, floor: u32) -> &mut BoxedMap {
        &mut self.map[floor as usize]
    }

    pub fn get_map_checked(&self, floor: u32) -> Option<&Map> {
        use std::ops::Deref;
        self.map.get(floor as usize).map(|a| a.deref())
    }

    pub fn get_map_mut_checked(&mut self, floor: u32) -> Option<&mut Map> {
        use std::ops::DerefMut;
        self.map.get_mut(floor as usize).map(|a| a.deref_mut())
    }

    pub(crate) fn add_map(&mut self, map: Map, map_random_id: u64) -> u32 {
        assert!((self.map.len() as u32) < self.max_floor);
        let floor = self.map.len() as u32;
        self.map.push(FileBox::new(map_random_id, map));
        floor
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_ref().map(|s| s.as_ref())
    }

    pub fn floor_num(&self) -> u32 {
        self.map.len() as u32
    }

    pub fn max_floor(&self) -> u32 {
        self.max_floor
    }

    pub fn map_exist(&self, floor: u32) -> bool {
        self.floor_num() > floor
    }

    pub fn visit_maps<F: FnMut(u32, &BoxedMap)>(&self, mut f: F) {
        for (i, map) in self.map.iter().enumerate() {
            f(i as u32, map)
        }
    }

    pub fn get_shop(&self, cid: CharaId) -> Option<&Shop> {
        self.shops.get(&cid)
    }

    pub fn get_shop_mut(&mut self, cid: CharaId) -> Option<&mut Shop> {
        self.shops.get_mut(&cid)
    }

    pub fn iter_shop_ids(
        &self,
    ) -> std::iter::Copied<std::collections::hash_map::Keys<'_, CharaId, Shop>> {
        self.shops.keys().copied()
    }

    pub fn iter_shops(&self) -> std::collections::hash_map::Iter<'_, CharaId, Shop> {
        self.shops.iter()
    }

    pub fn iter_shops_mut(&mut self) -> std::collections::hash_map::IterMut<'_, CharaId, Shop> {
        self.shops.iter_mut()
    }

    pub fn add_shop(&mut self, cid: CharaId, shop: Shop) {
        self.shops.insert(cid, shop);
    }
}

impl SiteContent {
    pub fn kind(&self) -> SiteKind {
        match self {
            SiteContent::AutoGenDungeon { .. } => SiteKind::AutoGenDungeon,
            SiteContent::Town { .. } => SiteKind::Town,
            SiteContent::Player { .. } => SiteKind::Player,
            SiteContent::Temp { .. } => SiteKind::Temp,
            SiteContent::Other { .. } => SiteKind::Other,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum SiteKind {
    AutoGenDungeon,
    Town,
    Player,
    Temp,
    Other,
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
            n: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DungeonKind(pub(crate) ArrayStringId);

impl DungeonKind {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum PlayerBaseKind {
    Normal,
}
