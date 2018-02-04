
/// Rules for character parameter calculation
#[derive(Serialize, Deserialize)]
pub struct Chara {
    /// Default value of CharaParams::view_range
    /// The actual value will be adjusted by character traits, and map attributes, etc.
    pub default_view_range: i32,
}

