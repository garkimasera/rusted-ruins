
/// Rules for quest

use std::collections::HashMap;
use common::gamedata::Race;

#[derive(Serialize, Deserialize)]
pub struct Quest {
    /// The probability of choose npc for monster slaying quest
    pub slay_race_probability: HashMap<Race, f32>,
}

