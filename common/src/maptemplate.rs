use crate::basic::N_TILE_IMG_LAYER;
#[cfg(feature = "global_state_obj")]
use crate::gamedata::map::TileLayers;
use crate::gamedata::ItemGen;
#[cfg(feature = "global_state_obj")]
use crate::objholder::ObjectIndex;
use crate::piece_pattern::*;
use arrayvec::ArrayVec;
use geom::*;
use std::ops::{Index, IndexMut};

/// Data for constructing one map
#[derive(Serialize, Deserialize)]
pub struct MapTemplateObject {
    pub id: String,
    pub w: u32,
    pub h: u32,
    /// Tile Id (String) <-> integer value conversion table
    pub tile_table: Vec<String>,
    pub tile: Array2d<TileLayersConverted>,
    /// Wall Id (String) <-> integer value conversion table
    pub wall_table: Vec<String>,
    pub wall: Array2d<ConvertedIdxPP>,
    /// Deco Id (String) <-> integer value conversion table
    pub deco_table: Vec<String>,
    pub deco: Array2d<Option<u32>>,
    pub boundary: MapTemplateBoundary,
    pub entrance: ArrayVec<[Vec2d; 4]>,
    pub items: Vec<(Vec2d, ItemGen)>,
    #[serde(default)]
    pub music: String,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct TileLayersConverted([ConvertedIdxPP; N_TILE_IMG_LAYER]);

impl Index<usize> for TileLayersConverted {
    type Output = ConvertedIdxPP;
    fn index(&self, index: usize) -> &ConvertedIdxPP {
        &self.0[index]
    }
}

impl IndexMut<usize> for TileLayersConverted {
    fn index_mut(&mut self, index: usize) -> &mut ConvertedIdxPP {
        &mut self.0[index]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MapTemplateBoundaryBehavior {
    None,
    NextFloor,
    PrevFloor,
    Exit,
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

#[cfg(feature = "global_state_obj")]
impl<T> ConvertableIndex for T
where
    T: ObjectIndex + Default,
{
    fn conv_into(self, table: &Vec<String>) -> u32 {
        use crate::gobj;
        let id = gobj::idx_to_id(self);
        table
            .iter()
            .position(|a| a == id)
            .expect("error while object index converting") as u32
    }

    fn conv_from(value: u32, table: &Vec<String>) -> T {
        use crate::gobj;
        let id = &table[value as usize];
        gobj::id_to_idx(id)
    }
}

#[cfg(feature = "global_state_obj")]
impl<T> IdxWithPiecePattern<T>
where
    T: ObjectIndex + Default,
{
    pub fn conv_into(self, table: &Vec<String>) -> ConvertedIdxPP {
        if let Some((idx, pp)) = self.get() {
            let cidx = idx.conv_into(table);
            let mut c = ConvertedIdxPP::from_raw_int(cidx + 1);
            c.set_piece_pattern(pp);
            c
        } else {
            ConvertedIdxPP::default()
        }
    }

    pub fn conv_from(c: ConvertedIdxPP, table: &Vec<String>) -> IdxWithPiecePattern<T> {
        if !c.is_empty() {
            let idx = T::conv_from(c.as_raw_int() - 1, table);
            IdxWithPiecePattern::with_piece_pattern(idx, c.piece_pattern())
        } else {
            IdxWithPiecePattern::default()
        }
    }
}

#[cfg(feature = "global_state_obj")]
impl TileLayers {
    pub fn conv_into(self, table: &Vec<String>) -> TileLayersConverted {
        let mut c = [ConvertedIdxPP::default(); N_TILE_IMG_LAYER];
        for i in 0..N_TILE_IMG_LAYER {
            c[i] = self[i].conv_into(table);
        }
        TileLayersConverted(c)
    }

    pub fn conv_from(c: TileLayersConverted, table: &Vec<String>) -> TileLayers {
        let mut o = [TileIdxPP::default(); N_TILE_IMG_LAYER];
        for i in 0..N_TILE_IMG_LAYER {
            o[i] = TileIdxPP::conv_from(c[i], table);
        }
        TileLayers(o)
    }
}
