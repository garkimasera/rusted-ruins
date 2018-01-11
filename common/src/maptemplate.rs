
use array2d::*;

/// Data for constructing one map
#[derive(Serialize, Deserialize)]
pub struct MapTemplateObject {
    pub id: String,
    pub w: u32,
    pub h: u32,
    /// Tile Id (String) <-> integer value conversion table
    pub tile_table: Vec<String>,
    pub tile: Array2d<u32>,
    /// Wall Id (String) <-> integer value conversion table
    pub wall_table: Vec<String>,
    pub wall: Array2d<Option<u32>>,
    /// Deco Id (String) <-> integer value conversion table
    pub deco_table: Vec<String>,
    pub deco: Array2d<Option<u32>>,
}

