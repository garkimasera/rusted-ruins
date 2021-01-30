use common::gobj::ObjIdxAsId;
use common::objholder::*;
use serde_with::serde_as;

/// Rules for wilderness map generation
use std::collections::HashMap;

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubBiomeDetail {}
