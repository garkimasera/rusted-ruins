#[derive(Serialize, Deserialize)]
pub struct Item {
    pub quality_level_factor: u32,
    pub rotten_item: String,
    pub rotten_item_gen_per_gram: u32,
}
