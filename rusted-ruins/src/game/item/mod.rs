
use rand::{Rng, thread_rng};
use common::gamedata::item::*;
use common::gobj;

/// Generate new item on dungeon floor
pub fn gen_dungeon_item(floor_level: u32) -> Box<Item> {
    let idx: ::common::objholder::ItemIdx = if thread_rng().gen_weighted_bool(3) {
        gobj::id_to_idx("!plank")
    } else {
        gobj::id_to_idx("!healing-potion")
    };

    let itemcontent = ItemContent::Object;
    let item = Item {
        idx: idx,
        content: itemcontent,
    };
    Box::new(item)
}

