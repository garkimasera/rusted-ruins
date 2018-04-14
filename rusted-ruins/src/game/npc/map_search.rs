//! Functions to search map information needed to determine NPC's behavior.

use common::gamedata::*;

/// Search nearest other character from cid.
/// If no character found, returns None.
pub fn search_nearest_chara(gd: &GameData, cid: CharaId) -> Option<CharaId> {
    let map = gd.get_current_map();
    let center = map.chara_pos(cid)?;

    let mut result_cid = None;
    let mut distance = i32::max_value();

    for p in map.tile.iter_idx() {
        if let Some(tile_cid) = map.tile[p].chara {
            if tile_cid != cid {
                let d = center.mdistance(p);
                if d < distance {
                    distance = d;
                    result_cid = Some(cid);
                }
            }
        }
    }
    
    result_cid
}

