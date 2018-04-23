
use array2d::*;
use basic::MAX_TILE_IMG_OVERLAP;
use gamedata::map::PiecePattern;

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
    pub wall: Array2d<Option<u32>>,
    /// Deco Id (String) <-> integer value conversion table
    pub deco_table: Vec<String>,
    pub deco: Array2d<Option<u32>>,
    pub boundary: MapTemplateBoundary,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct OverlappedTileConverted {
    pub piece_pattern: [PiecePattern; MAX_TILE_IMG_OVERLAP],
    pub idx: [u32; MAX_TILE_IMG_OVERLAP],
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

