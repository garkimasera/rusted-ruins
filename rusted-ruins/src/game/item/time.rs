use super::gen::gen_item_from_id;
use crate::game::extrait::ItemExt;
use common::gamedata::*;
use common::gobj;
use rules::RULES;

/// Update item time stamp for current map.
pub fn update_item_time(gd: &mut GameData) {
    let cids: Vec<CharaId> = gd.get_current_map().iter_charaid().copied().collect();

    // For characters
    for &cid in &cids {
        let ill = ItemListLocation::Chara { cid };

        update_item_list_time(gd, ill);
    }

    // For tiles
    let mid = gd.get_current_mapid();
    for pos in gd.get_current_map().tile.iter_idx() {
        let ill = ItemListLocation::OnMap { mid, pos };

        update_item_list_time(gd, ill);
    }
}

fn update_item_list_time(gd: &mut GameData, ill: ItemListLocation) {
    let item_list = gd.get_item_list_mut(ill);
    let mut items_to_add: Vec<(Item, u32)> = Vec::new();
    let mut items_to_remove: Vec<(u32, u32)> = Vec::new();

    for (i, (item, n)) in item_list.items.iter_mut().enumerate() {
        if item.update_time() {
            match process_item(item, *n) {
                UpdateTimeResult::None => (),
                UpdateTimeResult::Transform(item, n_gen, n_remove) => {
                    items_to_add.push((item, n_gen));
                    items_to_remove.push((i as u32, n_remove));
                }
            }
        }
    }

    for &(i, n) in &items_to_remove {
        gd.remove_item((ill, i), n);
    }

    let item_list = gd.get_item_list_mut(ill);
    for (item, n) in items_to_add.into_iter() {
        item_list.append(item, n);
    }
}

enum UpdateTimeResult {
    None,
    Transform(Item, u32, u32),
}

fn process_item(item: &mut Item, n: u32) -> UpdateTimeResult {
    let item_obj = gobj::get_obj(item.idx);

    if item_obj
        .attrs
        .iter()
        .any(|attr| matches!(attr, ItemObjAttr::Rot(_)))
    {
        let total_weight = item_obj.w * n;
        let n_rot_pile = std::cmp::max(total_weight / RULES.item.rotten_item_gen_per_gram, 1);

        let item = gen_item_from_id(&RULES.item.rotten_item, 1);

        return UpdateTimeResult::Transform(item, n_rot_pile, n);
    }

    UpdateTimeResult::None
}
