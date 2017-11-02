
pub mod builder;

use array2d::Vec2d;
use common::gamedata::GameData;
use common::gamedata::map::{Map, MapId};
use common::gamedata::site::DungeonKind;
use rand::{Rng, thread_rng};
use super::chara::creation::create_npc_chara;

pub fn gen_npcs(gd: &mut GameData, mid: MapId, n: u32, floor_level: u32) {
    for _ in 0..n {
        if let Some(p) = choose_empty_tile(gd.site.get_map(mid)) {
            let chara = create_npc_chara(DungeonKind::Cave, floor_level);
            trace!("Generate new npc {}", chara.name);
            gd.add_chara_to_map(chara, ::common::gamedata::chara::CharaKind::OnMap, mid, p);
        } else {
            trace!("Failed npc generating because empty tile not found");
            return;
        }
    }
}

/// Choose one empty tile in random
pub fn choose_empty_tile(map: &Map) -> Option<Vec2d> {
    const MAX_TRY: usize = 10;
    let mut rng = thread_rng();
    
    for _ in 0..MAX_TRY {
        let p = Vec2d::new(rng.gen_range(0, map.w) as i32, rng.gen_range(0, map.h) as i32);

        if map.tile[p].wall.is_none() && map.tile[p].chara.is_none() {
            return Some(p);
        }
    }

    // If random tile choosing is failed many times, count empty tiles and choose
    let n_empty_tile = map.tile.iter().filter(|t| t.wall.is_none() && t.chara.is_none()).count();
    if n_empty_tile == 0 {
        None
    } else {
        
        let r = rng.gen_range(0, n_empty_tile);
        let p = map.tile
            .iter_with_idx()
            .filter(|&(_, t)| t.wall.is_none() && t.chara.is_none())
            .skip(r)
            .next()
            .unwrap()
            .0;
        Some(p)
    }
}

