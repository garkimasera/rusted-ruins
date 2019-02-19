/// Used for town simulation
#[derive(Serialize, Deserialize)]
pub struct Town {
    /// The minimum number of shop items
    pub min_shop_items: u32,
    /// The maximum number of shop items
    pub max_shop_items: u32,
}
