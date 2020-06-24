//! Functions to search map information needed to determine NPC's behavior.

use crate::game::InfoGetter;
use common::gamedata::*;
use geom::*;

/// Search nearest other character from cid.
/// If f() returns false, skip the character.
/// If no character found, returns None.
pub fn search_nearest_chara<F>(gd: &GameData, cid: CharaId, mut f: F) -> Option<CharaId>
where
    F: FnMut(&GameData, CharaId, CharaId) -> bool,
{
    let map = gd.get_current_map();
    let center = map.chara_pos(cid)?;

    let mut result_cid = None;
    let mut distance = i32::max_value();

    for p in map.tile.iter_idx() {
        if let Some(tile_cid) = map.tile[p].chara {
            if tile_cid != cid && f(gd, cid, tile_cid) {
                let d = center.mdistance(p);
                if d < distance {
                    distance = d;
                    result_cid = Some(tile_cid);
                }
            }
        }
    }

    result_cid
}

/// Search the nearest hostile character
pub fn search_nearest_enemy(gd: &GameData, cid: CharaId) -> Option<CharaId> {
    search_nearest_chara(gd, cid, |gd, c0, c1| {
        gd.chara_relation(c0, c1) == Relationship::HOSTILE
    })
}

/// Returns direction to target chara
pub fn dir_to_chara(gd: &GameData, cid: CharaId, pos: Vec2d) -> Direction {
    if let Some(target_pos) = gd.chara_pos(cid) {
        geom::dir_by_2pos(pos, target_pos)
    } else {
        Direction::NONE
    }
}
