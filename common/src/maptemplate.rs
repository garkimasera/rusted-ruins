
use std::ops::{Index, IndexMut};
use array2d::*;
use basic::MAX_TILE_IMG_OVERLAP;
use piece_pattern::*;
#[cfg(feature="global_state_obj")]
use gamedata::map::OverlappedTile;
#[cfg(feature="global_state_obj")]
use objholder::ObjectIndex;

/// Data for constructing one map
#[derive(Serialize, Deserialize)]
pub struct MapTemplateObject {
    pub id: String,
    pub w: u32,
    pub h: u32,
    /// Tile Id (String) <-> integer value conversion table
    pub tile_table: Vec<String>,
    pub tile: Array2d<OverlappedTileConverted>,
    /// Wall Id (String) <-> integer value conversion table
    pub wall_table: Vec<String>,
    pub wall: Array2d<ConvertedIdxPP>,
    /// Deco Id (String) <-> integer value conversion table
    pub deco_table: Vec<String>,
    pub deco: Array2d<Option<u32>>,
    pub boundary: MapTemplateBoundary,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct OverlappedTileConverted([ConvertedIdxPP; MAX_TILE_IMG_OVERLAP]);

impl OverlappedTileConverted {
    pub fn len(&self) -> usize {
        for i in 0..MAX_TILE_IMG_OVERLAP {
            if self[i].is_empty() {
                return i;
            }
        }
        MAX_TILE_IMG_OVERLAP
    }
}

impl Index<usize> for OverlappedTileConverted {
    type Output = ConvertedIdxPP;
    fn index(&self, index: usize) -> &ConvertedIdxPP {
        &self.0[index]
    }
}

impl IndexMut<usize> for OverlappedTileConverted {
    fn index_mut(&mut self, index: usize) -> &mut ConvertedIdxPP {
        &mut self.0[index]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MapTemplateBoundaryBehavior {
    None, NextFloor, PrevFloor, RegionMap
}

impl Default for MapTemplateBoundaryBehavior {
    fn default() -> MapTemplateBoundaryBehavior {
        MapTemplateBoundaryBehavior::None
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MapTemplateBoundary {
    pub n: MapTemplateBoundaryBehavior,
    pub s: MapTemplateBoundaryBehavior,
    pub e: MapTemplateBoundaryBehavior,
    pub w: MapTemplateBoundaryBehavior,
}

/// Helper trait to convert between object index and u32 in maptemplate
pub trait ConvertableIndex {
    fn conv_into(self, table: &Vec<String>) -> u32;
    fn conv_from(value: u32, table: &Vec<String>) -> Self;
}    

#[cfg(feature="global_state_obj")]
impl<T> ConvertableIndex for T where T: ObjectIndex + Default {
    fn conv_into(self, table: &Vec<String>) -> u32 {
        use gobj;
        let id = gobj::idx_to_id(self);
        table
            .iter()
            .position(|a| a == id)
            .expect("error while object index converting") as u32
    }

    fn conv_from(value: u32, table: &Vec<String>) -> T {
        use gobj;
        let id = &table[value as usize];
        gobj::id_to_idx(id)
    }
}

#[cfg(feature="global_state_obj")]
impl<T> IdxWithPiecePattern<T> where T: ObjectIndex + Default {
    pub fn conv_into(self, table: &Vec<String>) -> ConvertedIdxPP {
        if !self.is_empty() {
            let cidx = self.idx.conv_into(table);
            ConvertedIdxPP {
                idx: cidx,
                piece_pattern: self.piece_pattern
            }
        } else {
            ConvertedIdxPP {
                idx: 0,
                piece_pattern: PiecePattern::EMPTY,
            }
        }
    }

    pub fn conv_from(c: ConvertedIdxPP, table: &Vec<String>) -> IdxWithPiecePattern<T> {
        if !c.is_empty() {
            let idx = T::conv_from(c.idx, table);
            IdxWithPiecePattern {
                idx: idx,
                piece_pattern: c.piece_pattern,
            }
        } else {
            IdxWithPiecePattern {
                idx: T::default(),
                piece_pattern: PiecePattern::EMPTY,
            }
        }
    }
}

#[cfg(feature="global_state_obj")]
impl OverlappedTile {
    pub fn conv_into(self, table: &Vec<String>) -> OverlappedTileConverted {
        let mut c = [ConvertedIdxPP::default(); MAX_TILE_IMG_OVERLAP];
        for i in 0..self.len() {
            c[i] = self[i].conv_into(table);
        }
        OverlappedTileConverted(c)
    }

    pub fn conv_from(c: OverlappedTileConverted, table: &Vec<String>) -> OverlappedTile {
        let mut o = [TileIdxPP::default(); MAX_TILE_IMG_OVERLAP];
        for i in 0..c.len() {
            o[i] = TileIdxPP::conv_from(c[i], table);
        }
        OverlappedTile(o)
    }
}
