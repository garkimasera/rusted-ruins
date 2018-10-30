
use std::ops::{Index, IndexMut};
use array2d::*;
use objholder::*;
use basic::{MAX_ITEM_FOR_DRAW, N_TILE_IMG_LAYER};
use gamedata::item::{Item, ItemList};
use gamedata::chara::CharaId;
use gamedata::site::SiteId;
use gamedata::region::RegionId;

pub use piece_pattern::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub w: u32,
    pub h: u32,
    pub tile: Array2d<TileInfo>,
    pub observed_tile: Array2d<ObservedTileInfo>,
    pub player_pos: Vec2d,
    pub entrance: Vec2d,
    /// Characters on this map
    charaid: Vec<CharaId>,
    /// This is drawed outer this map
    /// If this is None, nearest tile's infomation will be used
    pub outside_tile: Option<OutsideTileInfo>,
    pub boundary: MapBoundary,
}

/// Represents tile image layers
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TileLayers(pub [TileIdxPP; N_TILE_IMG_LAYER]);

impl Default for TileLayers {
    fn default() -> TileLayers {
        let tile = TileIdxPP {
            idx: TileIdx::default(),
            piece_pattern: PiecePattern::EMPTY,
        };
        let mut tile_layers = [tile; N_TILE_IMG_LAYER];
        tile_layers[0].piece_pattern = PiecePattern::SURROUNDED;
        TileLayers(tile_layers)
    }
}

impl From<TileIdx> for TileLayers {
    fn from(tile_idx: TileIdx) -> TileLayers {
        let mut overlapped_tile = TileLayers::default();
        overlapped_tile[0].idx = tile_idx;
        overlapped_tile
    }
}

impl Index<usize> for TileLayers {
    type Output = TileIdxPP;
    fn index(&self, index: usize) -> &TileIdxPP {
        &self.0[index]
    }
}

impl IndexMut<usize> for TileLayers {
    fn index_mut(&mut self, index: usize) -> &mut TileIdxPP {
        &mut self.0[index]
    }
}

impl TileLayers {
    pub fn main_tile(&self) -> TileIdx {
        let mut idx: Option<TileIdx> = None;
        for t in &self.0 {
            if !t.is_empty() {
                idx = Some(t.idx);
            }
        }
        if let Some(idx) = idx {
            idx
        } else {
            warn!("Every tile layer is empty. Use default index.");
            TileIdx::default()
        }
    }
}

/// This represents special objects on a tile. For example, stairs, doors, traps.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SpecialTileKind {
    None,
    Stairs {
        /// Stairs has the destination floor number
        dest_floor: u32,
        kind: StairsKind,
    },
    /// Site symbol on region map
    SiteSymbol {
        kind: SiteSymbolKind,
    }
}

impl SpecialTileKind {
    pub fn is_none(&self) -> bool {
        match *self {
            SpecialTileKind::None => true,
            _ => false,
        }
    }
}

impl Default for SpecialTileKind {
    fn default() -> SpecialTileKind {
        SpecialTileKind::None
            
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum StairsKind {
    UpStairs, DownStairs,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SiteSymbolKind {
    Cave, Ruin, Tower, Town, Village,
}

impl SpecialTileKind {
    /// Convert to id of SpecialTileObject
    pub fn obj_id(&self) -> Option<&'static str> {
        Some(match *self {
            SpecialTileKind::None => { return None; },
            SpecialTileKind::Stairs { kind, .. } => {
                match kind {
                    StairsKind::DownStairs => "!downstairs",
                    StairsKind::UpStairs => "!upstairs",
                }
            }
            SpecialTileKind::SiteSymbol { kind } => {
                match kind {
                    SiteSymbolKind::Cave =>    "!rm-cave",
                    SiteSymbolKind::Ruin =>    "!rm-ruin",
                    SiteSymbolKind::Tower =>   "!rm-tower",
                    SiteSymbolKind::Town =>    "!rm-town",
                    SiteSymbolKind::Village => "!rm-village",
                }
            }
        })
    }
}

/// If stairs or boundaries have this value, they are connected to region map
pub const FLOOR_OUTSIDE: u32 = 0xFFFFFFFF;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TileInfo {
    /// Base tile
    pub tile: TileLayers,
    /// If wall is presented, the tile is no walkable
    pub wall: WallIdxPP, 
    /// Decoration for this tile
    pub deco: Option<DecoIdx>,
    /// Items on this tile
    pub item_list: Option<ItemList>,
    pub chara: Option<CharaId>,
    pub special: SpecialTileKind,
}

/// The data for map drawing
/// These data will be updated every player turn based on player's view
#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct ObservedTileInfo {
    pub tile: Option<TileLayers>,
    pub wall: WallIdxPP,
    pub deco: Option<DecoIdx>,
    pub n_item: usize,
    pub items: [ItemIdx; MAX_ITEM_FOR_DRAW],
    pub special: SpecialTileKind,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct OutsideTileInfo {
    pub tile: TileIdx,
    pub wall: Option<WallIdx>,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MapBoundary {
    pub n: BoundaryBehavior,
    pub s: BoundaryBehavior,
    pub e: BoundaryBehavior,
    pub w: BoundaryBehavior,
}

/// Reperesents the floor that boundary connect to
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum BoundaryBehavior {
    None, Floor(u32), RegionMap, MapId(MapId, u32),
}

impl Default for BoundaryBehavior {
    fn default() -> BoundaryBehavior {
        BoundaryBehavior::None
    }
}
         
impl Default for TileInfo {
    fn default() -> TileInfo {
        TileInfo {
            tile: TileLayers::default(),
            wall: WallIdxPP::default(),
            deco: None,
            item_list: None,
            chara: None,
            special: SpecialTileKind::None,
        }
    }
}

impl Map {
    pub fn new(w: u32, h: u32) -> Map {
        Map {
            w: w, h: h,
            tile: Array2d::new(w, h, TileInfo::default()),
            observed_tile: Array2d::new(w, h, ObservedTileInfo::default()),
            player_pos: Vec2d::new(0, 0), entrance: Vec2d::new(0, 0),
            charaid: Vec::new(),
            outside_tile: None,
            boundary: MapBoundary::default(),
        }
    }

    pub fn width(&self) -> u32 {
        self.w
    }

    pub fn height(&self) -> u32 {
        self.h
    }

    pub fn size(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    pub fn get_chara<T: Into<Vec2d>>(&self, pos: T) -> Option<CharaId> {
        let pos = pos.into();
        if !self.is_inside(pos) { return None; }
        self.tile[pos].chara.clone()
    }

    pub fn iter_charaid(&self) -> ::std::slice::Iter<CharaId> {
        self.charaid.iter()
    }

    /// Return given pos is inside map or not
    #[inline]
    pub fn is_inside(&self, pos: Vec2d) -> bool {
        if pos.0 >= 0 && pos.1 >= 0 && (pos.0 as u32) < self.w && (pos.1 as u32) < self.h {
            true
        }else{
            false
        }
    }

    /// Get character position
    pub fn chara_pos(&self, cid: CharaId) -> Option<Vec2d> {
        for p in self.tile.iter_idx() {
            if self.tile[p].chara.as_ref() == Some(&cid) {
                return Some(p);
            }
        }
        None
    }

    /// Swaps characters on given tiles
    pub fn swap_chara(&mut self, a: Vec2d, b: Vec2d) -> bool {
        if !(self.is_inside(a) && self.is_inside(b)) {
            return false
        }
        use std::mem::replace;
        let temp0 = replace(&mut self.tile[a].chara, None);
        let temp1 = replace(&mut self.tile[b].chara, None);
        replace(&mut self.tile[a].chara, temp1);
        replace(&mut self.tile[b].chara, temp0);
        true
    }

    /// Locate a character at given position.
    /// If the new position is not empty, this function will fail and return false
    pub fn locate_chara(&mut self, cid: CharaId, pos: Vec2d) -> bool {
        if !self.charaid.iter().any(|a| *a == cid) {
            self.charaid.push(cid);
        }
        
        if self.tile[pos].chara.is_some() { return false; }
            
        if let Some(old_pos) = self.chara_pos(cid) {
            self.tile[old_pos].chara = None;
        }
        self.tile[pos].chara = Some(cid);
        true
    }

    /// Locate item at the specified tile.
    /// Usually should use GameData functions instead of this to move and append item.
    pub fn locate_item(&mut self, item: Item, pos: Vec2d, n: u32) {
        if let Some(ref mut item_list) = self.tile[pos].item_list {
            item_list.append(item, n);
            return;
        }
        let mut item_list = ItemList::new();
        item_list.append(item, n);
        self.tile[pos].item_list = Some(item_list)
    }

    pub(crate) fn search_empty_onmap_charaid_n(&self) -> u32 {
        'i_loop:
        for i in 0.. {
            for cid in self.charaid.iter() {
                if let CharaId::OnMap { n, .. } = *cid {
                    if i == n { continue 'i_loop; }
                }
            }
            return i;
        }
        panic!()
    }

    pub(crate) fn remove_chara(&mut self, cid: CharaId) {
        let pos = self.chara_pos(cid).unwrap();
        self.tile[pos].chara = None;

        if let CharaId::OnMap { .. } = cid {
            let i = self.charaid.iter().enumerate().find(|&(_, cid_o)| *cid_o == cid).unwrap().0;
            let removed_cid = self.charaid.swap_remove(i);
            assert!(removed_cid == cid);
        }
    }

    /// Get tile index with extrapolation
    /// If pos is outside map and self.outside_tile has value, returns it.
    /// If pos is outside map and self.outside_tile is None, returns the nearest tile.
    pub fn get_tile_extrapolated(&self, pos: Vec2d) -> TileLayers {
        if self.is_inside(pos) {
            return self.tile[pos].tile;
        }
        if let Some(outside_tile) = self.outside_tile {
            outside_tile.tile.into()
        } else {
            self.tile[self.nearest_existent_tile(pos)].tile
        }
    }

    pub fn get_wall_extrapolated(&self, pos: Vec2d) -> WallIdxPP {
        if self.is_inside(pos) {
            return self.tile[pos].wall;
        }
        if let Some(outside_tile) = self.outside_tile {
            if let Some(idx) = outside_tile.wall {
                WallIdxPP {
                    idx: idx,
                    piece_pattern: PiecePattern::SURROUNDED,
                }
            } else {
                WallIdxPP::default()
            }
        } else {
            self.tile[self.nearest_existent_tile(pos)].wall
        }
    }

    /// Calculate the position of nearest and exsitent tile
    pub fn nearest_existent_tile(&self, pos: Vec2d) -> Vec2d {
        if self.is_inside(pos) {
            return pos;
        }
        let (w, h) = (self.w as i32, self.h as i32);
        let x = if pos.0 < 0 { 0 } else if pos.0 >= w { w - 1 } else { pos.0 };
        let y = if pos.1 < 0 { 0 } else if pos.1 >= h { h - 1 } else { pos.1 };
        Vec2d::new(x, y)
    }

    /// Search stairs that is connected to given floor
    pub fn search_stairs(&self, floor: u32) -> Option<Vec2d> {
        for (p, tile) in self.tile.iter_with_idx() {
            if let SpecialTileKind::Stairs { dest_floor, .. } = tile.special {
                if dest_floor == floor {
                    return Some(p);
                }
            }
        }
        None
    }

    /// Returns boundart when player move from 'pos' to 'dir' direction
    /// If its destination tile is inside map, return None
    pub fn get_boundary_by_tile_and_dir(&self, pos: Vec2d, dir: Direction) -> Option<BoundaryBehavior> {
        let dest_pos = pos + dir.as_vec();
        if dest_pos.0 < 0 {
            Some(self.boundary.n)
        } else if dest_pos.0 >= self.h as i32 {
            Some(self.boundary.s)
        } else if dest_pos.1 < 0 {
            Some(self.boundary.w)
        } else if dest_pos.1 >= self.w as i32 {
            Some(self.boundary.e)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum MapId {
    SiteMap { sid: SiteId, floor: u32 },
    RegionMap { rid: RegionId },
}

impl MapId {
    pub fn site_first_floor(sid: SiteId) -> MapId {
        MapId::SiteMap { sid, floor: 0 }
    }

    pub fn set_floor(self, floor: u32) -> MapId {
        match self {
            MapId::SiteMap { sid, .. } => {
                MapId::SiteMap { sid, floor }
            }
            _ => panic!("Invalid operation on MapId::RegionId")
        }
    }

    #[inline]
    pub fn rid(self) -> RegionId {
        match self {
            MapId::SiteMap { sid, .. } => sid.rid,
            MapId::RegionMap { rid } => rid,
        }
    }

    #[inline]
    pub fn sid(self) -> SiteId {
        match self {
            MapId::SiteMap { sid, .. } => sid,
            _ => panic!("Invalid operation on MapId::RegionId")
        }
    }

    #[inline]
    /// Get floor number of this map
    /// If the map is region map, returns FLOOR_OUTSIDE
    pub fn floor(self) -> u32 {
        match self {
            MapId::SiteMap { floor, .. } => floor,
            MapId::RegionMap { .. } => FLOOR_OUTSIDE,
        }
    }

    pub fn is_region_map(self) -> bool {
        match self {
            MapId::RegionMap { .. } => true,
            _ => false,
        }
    }
}

impl Default for MapId {
    fn default() -> MapId {
        MapId::SiteMap { sid: SiteId::default(), floor: 0 }
    }
}

impl From<RegionId> for MapId {
    fn from(rid: RegionId) -> MapId {
        MapId::RegionMap { rid }
    }
}

