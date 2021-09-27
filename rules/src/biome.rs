use super::Rule;
use common::gobj::ObjIdxAsId;
use common::objholder::*;
use serde_with::serde_as;
use std::collections::HashMap;

/// Rules for wilderness map generation
#[derive(Debug, Serialize, Deserialize)]
pub struct Biomes {
    pub biomes: HashMap<String, BiomeDetail>,
    pub sub_biomes: HashMap<String, SubBiomeDetail>,
}

impl Rule for Biomes {
    const NAME: &'static str = "biomes";

    fn append(&mut self, other: Self) {
        for (k, v) in other.biomes.into_iter() {
            self.biomes.insert(k, v);
        }
        for (k, v) in other.sub_biomes.into_iter() {
            self.sub_biomes.insert(k, v);
        }
    }
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
