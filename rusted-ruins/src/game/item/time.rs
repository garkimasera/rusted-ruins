use crate::game::extrait::ItemExt;
use common::gamedata::*;

/// Update item time stamp for current map.
pub fn update_item_time(gd: &mut GameData) {
    let cids: Vec<CharaId> = gd.get_current_map().iter_charaid().copied().collect();

    // For characters
    for &cid in &cids {
        let ill = ItemListLocation::Chara { cid };
        let il = gd.get_item_list_mut(ill);

        for (item, _) in &mut il.items {
            item.update_time();
        }
    }

    // For tiles
    let mid = gd.get_current_mapid();
    for pos in gd.get_current_map().tile.iter_idx() {
        let ill = ItemListLocation::OnMap { mid, pos };

        let il = gd.get_item_list_mut(ill);

        for (item, _) in &mut il.items {
            item.update_time();
        }
    }
}
