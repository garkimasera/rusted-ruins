/// Rules for calculation related to combat.
#[derive(Serialize, Deserialize)]
pub struct Combat {
    pub throw_range_factor: u32,
    pub throw_range_max: u32,
}
