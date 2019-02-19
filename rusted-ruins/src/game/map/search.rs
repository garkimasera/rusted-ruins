//! Functions to search objects in a map

use crate::game::view::calc_visual_distance;
use common::gamedata::*;

/// Search the nearest chara's position that has given Relationship on the current map.
pub fn search_nearest_target(
    gd: &GameData,
    center_cid: CharaId,
    rel: Relationship,
) -> Option<CharaId> {
    let map = gd.get_current_map();
    let chara = gd.chara.get(center_cid);

    let center = if let Some(center) = map.chara_pos(center_cid) {
        center
    } else {
        return None;
    };

    let mut target_cid = None;
    let mut min_distance = i32::max_value();

    for cid in map.iter_charaid() {
        if center_cid == *cid {
            continue;
        }
        if gd.chara.get(*cid).rel.relative(chara.rel) != rel {
            continue;
        }

        let pos = if let Some(pos) = map.chara_pos(*cid) {
            pos
        } else {
            continue;
        };

        let visual_distance = if let Some(visual_distance) = calc_visual_distance(map, center, pos)
        {
            visual_distance
        } else {
            continue;
        };

        if visual_distance <= chara.attr.view_range && visual_distance < min_distance {
            target_cid = Some(*cid);
            min_distance = visual_distance;
        }
    }

    target_cid
}
