/// Rules for wilderness map generation
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Biome {
    biomes: HashMap<String, BiomeDetail>,
    sub_biomes: HashMap<String, SubBiomeDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeDetail {
    main_tile: String,
    main_wall: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubBiomeDetail {}
