use std::collections::HashMap;

use common::gamedata::*;

use crate::Rule;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub quality_level_factor: u32,
    pub rotten_item: String,
    pub rotten_item_gen_per_gram: u32,
    pub build_obj_default: BuildObj,

    // About slots
    pub ability_slot_required_quality: HashMap<EquipSlotKind, i32>,
    pub core_slot_required_quality: HashMap<EquipSlotKind, i32>,
    pub extend_slot_required_quality: HashMap<EquipSlotKind, Vec<i32>>,
    pub slot_install_cost_base: HashMap<ModuleSlotKind, i64>,
    pub slot_install_cost_factor: HashMap<ModuleSlotKind, f32>,
}

impl Rule for Item {
    const NAME: &'static str = "item";
}
