use crate::basic::ARRAY_STR_ID_LEN;
use crate::basic::{MAX_ITEM_FOR_DRAW, N_TILE_IMG_LAYER};
use crate::gamedata::chara::{Chara, CharaId};
use crate::gamedata::item::{Item, ItemList};
use crate::gamedata::region::RegionId;
use crate::gamedata::site::{DungeonKind, SiteId};
use crate::objholder::*;
use arrayvec::{ArrayString, ArrayVec};
use geom::*;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub use crate::piece_pattern::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Map {
    pub w: u32,
    pub h: u32,
    pub tile: Array2d<TileInfo>,
    pub observed_tile: Array2d<ObservedTileInfo>,
    pub player_pos: Vec2d,
    pub entrance: ArrayVec<Vec2d, 4>,
    /// Characters on this map
    charaid: Vec<CharaId>,
    /// Character data on this map. The current map's charas are moved to CharaHolder temporary.
    /// In order to reduce the size of main save file.
    pub(crate) charas: Option<HashMap<CharaId, Chara>>,
    /// This is drawed outer this map
    /// If this is None, nearest tile's infomation will be used
    pub outside_tile: Option<OutsideTileInfo>,
    pub boundary: MapBoundary,
    pub music: String,
}

/// Represents tile image layers
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct TileLayers(pub [TileIdxPp; N_TILE_IMG_LAYER]);

impl Default for TileLayers {
    fn default() -> TileLayers {
        let mut tile_layers = [TileIdxPp::default(); N_TILE_IMG_LAYER];
        tile_layers[0] = TileIdxPp::new(TileIdx::default());
        TileLayers(tile_layers)
    }
}

impl From<TileIdx> for TileLayers {
    fn from(tile_idx: TileIdx) -> TileLayers {
        let mut overlapped_tile = TileLayers::default();
        overlapped_tile[0] = TileIdxPp::new(tile_idx);
        overlapped_tile
    }
}

impl Index<usize> for TileLayers {
    type Output = TileIdxPp;
    fn index(&self, index: usize) -> &TileIdxPp {
        &self.0[index]
    }
}

impl IndexMut<usize> for TileLayers {
    fn index_mut(&mut self, index: usize) -> &mut TileIdxPp {
        &mut self.0[index]
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
    },
}

impl SpecialTileKind {
    pub fn is_none(&self) -> bool {
        matches!(*self, SpecialTileKind::None)
    }
}

impl Default for SpecialTileKind {
    fn default() -> SpecialTileKind {
        SpecialTileKind::None
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum StairsKind {
    UpStairs,
    DownStairs,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SiteSymbolKind(ArrayString<ARRAY_STR_ID_LEN>);

impl From<&str> for SiteSymbolKind {
    fn from(kind: &str) -> Self {
        SiteSymbolKind(ArrayString::from(kind).expect("too long site symbol kind name"))
    }
}

impl SpecialTileKind {
    /// Convert to id of SpecialTileObject
    pub fn obj_id(&self) -> Option<&str> {
        Some(match *self {
            SpecialTileKind::None => {
                return None;
            }
            SpecialTileKind::Stairs { kind, .. } => match kind {
                StairsKind::DownStairs => "!downstairs",
                StairsKind::UpStairs => "!upstairs",
            },
            SpecialTileKind::SiteSymbol { ref kind } => kind.0.as_str(),
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
    pub wall: WallIdxPp,
    /// Wall HP
    pub wall_hp: u16,
    /// Decoration for this tile
    pub deco: Option<DecoIdx>,
    /// Items on this tile
    pub item_list: ItemList,
    pub chara: Option<CharaId>,
    pub special: SpecialTileKind,
}

/// The data for map drawing
/// These data will be updated every player turn based on player's view
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ObservedTileInfo {
    /// Tile is observed or not
    pub tile: bool,
    pub wall: WallIdxPp,
    pub deco: Option<DecoIdx>,
    pub items: ArrayVec<(ItemIdx, u32), MAX_ITEM_FOR_DRAW>,
    pub special: SpecialTileKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutsideTileInfo {
    pub tile: TileLayers,
    pub wall: Option<WallIdx>,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MapBoundary {
    pub n: Option<Destination>,
    pub s: Option<Destination>,
    pub e: Option<Destination>,
    pub w: Option<Destination>,
}

impl MapBoundary {
    pub fn from_same_destination(d: Destination) -> Self {
        Self {
            n: Some(d),
            s: Some(d),
            e: Some(d),
            w: Some(d),
        }
    }
}

/// Reperesents the floor that boundary connect to
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Destination {
    Floor(u32),
    Exit,
    MapIdWithPos(MapId, Vec2d),
    MapIdWithEntrance(MapId, u32),
    MapId(MapId),
}

impl Default for TileInfo {
    fn default() -> TileInfo {
        TileInfo {
            tile: TileLayers::default(),
            wall: WallIdxPp::default(),
            wall_hp: 0,
            deco: None,
            item_list: ItemList::default(),
            chara: None,
            special: SpecialTileKind::None,
        }
    }
}

impl TileInfo {
    pub fn main_tile(&self) -> TileIdx {
        if let Some(idx) = self.tile.0.iter().filter_map(|idxpp| idxpp.idx()).last() {
            idx
        } else {
            warn!("Every tile layer is empty. Use default index.");
            TileIdx::default()
        }
    }
}

impl Map {
    pub fn new(w: u32, h: u32) -> Map {
        Map {
            w,
            h,
            tile: Array2d::new(w, h, TileInfo::default()),
            observed_tile: Array2d::new(w, h, ObservedTileInfo::default()),
            player_pos: Vec2d(0, 0),
            entrance: ArrayVec::new(),
            charaid: Vec::new(),
            charas: Some(HashMap::new()),
            outside_tile: None,
            boundary: MapBoundary::default(),
            music: String::default(),
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
        if !self.is_inside(pos) {
            return None;
        }
        self.tile[pos].chara
    }

    pub fn iter_charaid(&self) -> std::slice::Iter<CharaId> {
        self.charaid.iter()
    }

    /// Return given pos is inside map or not
    #[inline]
    pub fn is_inside(&self, pos: Vec2d) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && (pos.0 as u32) < self.w && (pos.1 as u32) < self.h
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
            return false;
        }
        use std::mem::replace;
        let temp0 = replace(&mut self.tile[a].chara, None);
        let temp1 = replace(&mut self.tile[b].chara, None);
        let _ = replace(&mut self.tile[a].chara, temp1);
        let _ = replace(&mut self.tile[b].chara, temp0);
        true
    }

    /// Locate a character at given position.
    /// If the new position is not empty, this function will fail and return false
    pub fn locate_chara(&mut self, cid: CharaId, pos: Vec2d) -> bool {
        if !self.charaid.iter().any(|a| *a == cid) {
            self.charaid.push(cid);
        }

        if self.tile[pos].chara.is_some() {
            return false;
        }

        if let Some(old_pos) = self.chara_pos(cid) {
            self.tile[old_pos].chara = None;
        }
        self.tile[pos].chara = Some(cid);
        true
    }

    /// Locate item at the specified tile.
    /// Usually should use GameData functions instead of this to move and append item.
    pub fn locate_item(&mut self, item: Item, pos: Vec2d, n: u32) {
        self.tile[pos].item_list.append(item, n);
    }

    pub(crate) fn search_empty_onmap_charaid_n(&self) -> u32 {
        'i_loop: for i in 0.. {
            for cid in self.charaid.iter() {
                if let CharaId::OnMap { n, .. } = *cid {
                    if i == n {
                        continue 'i_loop;
                    }
                }
            }
            return i;
        }
        panic!()
    }

    pub(crate) fn remove_chara(&mut self, cid: CharaId) {
        let pos = self.chara_pos(cid).unwrap();
        self.tile[pos].chara = None;

        let i = self
            .charaid
            .iter()
            .enumerate()
            .find(|&(_, cid_o)| *cid_o == cid)
            .unwrap()
            .0;
        let removed_cid = self.charaid.swap_remove(i);
        debug_assert!(removed_cid == cid);
    }

    /// Set tile. If layer is None, set to the last element of layers default.
    #[cfg(feature = "global_state_obj")]
    pub fn set_tile(&mut self, pos: Vec2d, tile_idx: TileIdx, layer: Option<usize>) {
        self.tile[pos].tile[layer.unwrap_or(N_TILE_IMG_LAYER - 1)] = TileIdxPp::new(tile_idx);
    }

    /// Set wall and wall hp to given pos
    #[cfg(feature = "global_state_obj")]
    pub fn set_wall(&mut self, pos: Vec2d, wall_idx: WallIdx) {
        let wall_obj = crate::gobj::get_obj(wall_idx);
        self.tile[pos].wall = WallIdxPp::new(wall_idx);
        self.tile[pos].wall_hp = wall_obj.hp;
        self.reset_wall_pp(pos, pos);
    }

    /// Erase wall
    #[cfg(feature = "global_state_obj")]
    pub fn erase_wall(&mut self, pos: Vec2d) {
        self.tile[pos].wall = WallIdxPp::empty();
        self.tile[pos].wall_hp = 0;
        self.reset_wall_pp(pos, pos);
    }

    /// Reset wall piece patterns
    #[cfg(feature = "global_state_obj")]
    pub fn reset_wall_pp(&mut self, top_left: Vec2d, bottom_right: Vec2d) {
        let rect_iter = RectIter::new(top_left - (1, 1), bottom_right + (1, 1));

        for p in rect_iter {
            if !self.is_inside(p) {
                continue;
            }
            let wall_idx = if let Some(wall_idx) = self.tile[p].wall.idx() {
                wall_idx
            } else {
                continue;
            };

            let ppf = PiecePatternFlags::from_fn(p, |p| {
                if self.is_inside(p) {
                    self.tile[p].wall.idx() == Some(wall_idx)
                } else {
                    false
                }
            });
            let wall_obj = crate::gobj::get_obj(wall_idx);
            let wallpp = WallIdxPp::with_piece_pattern(
                wall_idx,
                ppf.to_piece_pattern(wall_obj.img.n_pattern),
            );
            self.tile[p].wall = wallpp;
        }
    }

    /// Get tile index with extrapolation
    /// If pos is outside map and self.outside_tile has value, returns it.
    /// If pos is outside map and self.outside_tile is None, returns the nearest tile.
    pub fn get_tile_extrapolated(&self, pos: Vec2d) -> &TileLayers {
        if self.is_inside(pos) {
            return &self.tile[pos].tile;
        }
        if let Some(outside_tile) = self.outside_tile.as_ref() {
            &outside_tile.tile
        } else {
            &self.tile[self.nearest_existent_tile(pos)].tile
        }
    }

    pub fn get_wall_extrapolated(&self, pos: Vec2d) -> WallIdxPp {
        if self.is_inside(pos) {
            return self.tile[pos].wall;
        }
        if let Some(outside_tile) = self.outside_tile.as_ref() {
            if let Some(idx) = outside_tile.wall {
                WallIdxPp::new(idx)
            } else {
                WallIdxPp::default()
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
        let x = if pos.0 < 0 {
            0
        } else if pos.0 >= w {
            w - 1
        } else {
            pos.0
        };
        let y = if pos.1 < 0 {
            0
        } else if pos.1 >= h {
            h - 1
        } else {
            pos.1
        };
        Vec2d(x, y)
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
    pub fn get_boundary_by_tile_and_dir(&self, pos: Vec2d, dir: Direction) -> Option<Destination> {
        let dest_pos = pos + dir.as_vec();
        if dest_pos.1 < 0 {
            self.boundary.n
        } else if dest_pos.1 >= self.h as i32 {
            self.boundary.s
        } else if dest_pos.0 < 0 {
            self.boundary.w
        } else if dest_pos.0 >= self.w as i32 {
            self.boundary.e
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
            MapId::SiteMap { sid, .. } => MapId::SiteMap { sid, floor },
            _ => panic!("Invalid operation on MapId::RegionId"),
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
            _ => panic!("Invalid operation on MapId::RegionId"),
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
        matches!(self, MapId::RegionMap { .. })
    }
}

impl Default for MapId {
    fn default() -> MapId {
        MapId::SiteMap {
            sid: SiteId::default(),
            floor: 0,
        }
    }
}

impl From<RegionId> for MapId {
    fn from(rid: RegionId) -> MapId {
        MapId::RegionMap { rid }
    }
}
