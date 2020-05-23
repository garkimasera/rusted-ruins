/// Rules for quest
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Quest {
    /// The probability of choose npc for monster slaying quest
    pub slay_race_probability: HashMap<String, f32>,
}
