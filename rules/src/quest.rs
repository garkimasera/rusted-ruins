use crate::Rule;

/// Rules for quest
#[derive(Serialize, Deserialize)]
pub struct Quest {
    pub duplicate_factor: f32,
    pub update_duration_days: u32,
}

impl Rule for Quest {
    const NAME: &'static str = "quest";
}
