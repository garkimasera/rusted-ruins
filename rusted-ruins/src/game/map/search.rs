//! Functions to search objects in a map

use crate::game::InfoGetter;
use common::gamedata::*;
use geom::*;

/// Search specified facility item.
pub fn search_facility<'a>(gd: &'a GameData, facility_type: &str) -> Option<&'a Item> {
    let il = gd.get_item_list(ItemListLocation::PLAYER);
    let mut facility_item = None;
    let mut quality = std::i8::MIN;

    // Search player character inventory.
    for (item, _) in il.iter() {
        if let Some((ty, q)) = item
            .obj()
            .attrs
            .iter()
            .filter_map(|attr| match attr {
                ItemObjAttr::Facility { ty, quality } => Some((ty, quality)),
                _ => None,
            })
            .next()
        {
            if facility_type == ty && *q > quality {
                facility_item = Some(item);
                quality = *q;
            }
        }
    }

    let player_pos = gd.player_pos();
    let rect_iter = RectIter::new(player_pos + (-1, -1), player_pos + (1, 1));
    let map = gd.get_current_map();
    let mid = gd.get_current_mapid();

    // Search player surrounding tiles.
    for pos in rect_iter {
        if !map.is_inside(pos) {
            continue;
        }
        let ill = ItemListLocation::OnMap { mid, pos };
        let il = gd.get_item_list(ill);
        for (item, _) in il.iter() {
            if let Some((ty, q)) = item
                .obj()
                .attrs
                .iter()
                .filter_map(|attr| match attr {
                    ItemObjAttr::Facility { ty, quality } => Some((ty, quality)),
                    _ => None,
                })
                .next()
            {
                if facility_type == ty && *q > quality {
                    facility_item = Some(item);
                    quality = *q;
                }
            }
        }
    }

    facility_item
}

use crate::game::view::calc_visual_distance;

/// Search the nearest chara's position that has given Relationship on the current map.
pub fn search_nearest_target(
    gd: &GameData,
    center_cid: CharaId,
    rel: Relationship,
    limit_distance: i32,
) -> Option<CharaId> {
    let map = gd.get_current_map();
    let chara = gd.chara.get(center_cid);

    let center = map.chara_pos(center_cid)?;

    let mut target_cid = None;
    let mut min_distance = limit_distance;

    for &cid in map.iter_charaid() {
        if center_cid == cid {
            continue;
        }
        if gd.chara_relation(center_cid, cid) != rel {
            continue;
        }

        let pos = if let Some(pos) = map.chara_pos(cid) {
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
            target_cid = Some(cid);
            min_distance = visual_distance;
        }
    }

    target_cid
}
