use crate::game::InfoGetter;
use common::gamedata::*;
use common::gobj;
use common::objholder::ItemIdx;

pub fn harvest_item(gd: &mut GameData, il: ItemLocation) {
    let item = gd.remove_item_and_get(il, 1);
    let item_idx = item.idx;
    let item_obj = gobj::get_obj(item_idx);

    let harvest = item_obj
        .harvest
        .as_ref()
        .expect("Tried to harvest item that is not harvestable");

    let target_item_idx: ItemIdx = gobj::id_to_idx(&harvest.target_item);
    let target_item = super::gen::gen_item_from_idx(target_item_idx);
    gd.add_item_on_tile(gd.player_pos(), target_item, harvest.n_yield);
}
