
use std::collections::HashMap;
use common::gamedata::chara::Race;
use common::gamedata::item::EquipSlotKind;

/// Rules for character generation
#[derive(Serialize, Deserialize)]
pub struct CharaGen {
    /// Items on a map is generated with a probability of 1/ item_gen_probability
    pub default_equip_slots: HashMap<Race, Vec<(EquipSlotKind, u8)>>,
}

