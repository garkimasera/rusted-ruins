
use array2d::*;

/// Data for constructing one map
#[derive(Serialize, Deserialize)]
pub struct MapTemplateObject {
    pub id: String,
    pub w: u32,
    pub h: u32,
    /// Tile Id (String) <-> integer value conversion table
    pub tile_table: Vec<String>,
    /// Array of tuples of base tile id and frame number
    pub base_tile: Array2d<Option<(u32, u8)>>,
    pub tile: Array2d<u32>,
    /// Wall Id (String) <-> integer value conversion table
    pub wall_table: Vec<String>,
    pub wall: Array2d<Option<u32>>,
    /// Deco Id (String) <-> integer value conversion table
    pub deco_table: Vec<String>,
    pub deco: Array2d<Option<u32>>,
    pub boundary: MapTemplateBoundary,
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

