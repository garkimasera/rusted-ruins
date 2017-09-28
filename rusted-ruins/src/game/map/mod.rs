
mod builder;

use array2d::*;
use common::objholder::*;
use common::item::Inventory;
use game::CharaId;

pub use self::builder::MapBuilder;

pub struct Map {
    pub w: u32,
    pub h: u32,
    pub tile: Array2d<TileInfo>,
    pub player_pos: Vec2d,
}

#[derive(Clone, Debug)]
pub struct TileInfo {
    pub tile: TileIdx, // Basic tile type
    pub wall: Option<WallIdx>, // If wall is presented, the tile is no walkable
    pub items: Option<Inventory>,
    chara: Option<CharaId>,
}

impl Default for TileInfo {
    fn default() -> TileInfo {
        TileInfo {
            tile: TileIdx(0),
            wall: None,
            items: None,
            chara: None,
        }
    }
}

impl Map {
    pub fn new(w: u32, h: u32) -> Map {
        Map {
            w: w, h: h, tile: Array2d::new(w, h, TileInfo::default()),
            player_pos: Vec2d::new(0, 0),
        }
    }

    pub fn get_chara<T: Into<Vec2d>>(&self, pos: T) -> Option<CharaId> {
        self.tile[pos.into()].chara
    }

    pub fn is_movable(&self, pos: Vec2d) -> bool {
        if !self.is_inside(pos) {
            return false;
        }

        if self.tile[pos].wall.is_some() {
            false
        }else{
            true
        }
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

    /// Add one character on this map.
    pub fn add_character(&mut self, pos: Vec2d, id: CharaId) {
        self.tile[pos].chara = Some(id);
    }

    /// Get character position
    pub fn chara_pos(&self, id: CharaId) -> Option<Vec2d> {
        for p in self.tile.iter_idx() {
            if self.tile[p].chara == Some(id) {
                return Some(p);
            }
        }
        None
    }

    pub fn move_chara(&mut self, id: CharaId, dir: Direction) -> bool {
        use std::mem::replace;
        if let Some(p) = self.chara_pos(id) {
            let new_p = p + dir.as_vec();
            if self.is_movable(p + dir.as_vec()) { // Swap charas of the two tiles
                let temp0 = replace(&mut self.tile[p].chara,     None);
                let temp1 = replace(&mut self.tile[new_p].chara, None);
                replace(&mut self.tile[p].chara,     temp1);
                replace(&mut self.tile[new_p].chara, temp0);
                true
            }else{
                false
            }
        }else{
            false
        }
    }
}
    
