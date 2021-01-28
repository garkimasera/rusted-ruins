/// Rules for wilderness map generation
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Biome {
    pub biomes: HashMap<String, BiomeDetail>,
    pub sub_biomes: HashMap<String, SubBiomeDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeDetail {
    pub tile: String,
    pub wall: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubBiomeDetail {}
