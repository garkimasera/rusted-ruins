use common::gobj::ObjIdxAsId;
use common::objholder::*;
use serde_with::serde_as;

use std::collections::HashMap;

/// Rules for wilderness map generation
#[derive(Debug, Serialize, Deserialize)]
pub struct Biome {
    pub biomes: HashMap<String, BiomeDetail>,
    pub sub_biomes: HashMap<String, SubBiomeDetail>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeDetail {
    #[serde_as(as = "ObjIdxAsId")]
    pub tile: TileIdx,
    #[serde_as(as = "ObjIdxAsId")]
    pub wall: WallIdx,
    #[serde_as(as = "Vec<(ObjIdxAsId, _)>")]
    pub plants: Vec<(ItemIdx, f32)>,
    #[serde_as(as = "Vec<(ObjIdxAsId, _)>")]
    pub items: Vec<(ItemIdx, f32)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubBiomeDetail {}
