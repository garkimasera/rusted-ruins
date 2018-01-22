
pub mod builder;
pub mod from_template;

use array2d::Vec2d;
use common::gamedata::GameData;
use common::gamedata::map::{Map, MapId};
use common::gamedata::chara::CharaId;
use common::gamedata::site::DungeonKind;
use common::gamedata::item::ItemList;
use rand::{Rng, thread_rng};
use super::chara::creation::create_npc_chara;
use super::item::gen::gen_dungeon_item;
use rules::RULES;

/// Switch current map to the specified map
pub fn switch_map(gd: &mut GameData, mid: MapId) {
    let prev_mid = gd.get_current_mapid();
    gd.set_current_mapid(mid);

    let new_player_pos = if mid.is_region_map && !prev_mid.is_region_map
        && mid.sid.rid == prev_mid.sid.rid { // Exit from a site to region map
        gd.region.get_site_pos(mid.sid)
    } else {
        gd.get_current_map().entrance
    };
    
    gd.get_current_map_mut().locate_chara(CharaId::Player, new_player_pos);
}

pub fn gen_npcs(gd: &mut GameData, mid: MapId, n: u32, floor_level: u32) {
    for _ in 0..n {
        if let Some(p) = choose_empty_tile(gd.region.get_map(mid)) {
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

/// Locate some items for a new map
pub fn gen_items(gd: &mut GameData, mid: MapId) {
    let map = gd.region.get_map_mut(mid);

    for p in map.tile.iter_idx() {
        let tile = &mut map.tile[p];
        if tile.wall.is_some() { continue; } // Skip tile with wall

        let mut item_list = ItemList::new(10);

        if thread_rng().gen_weighted_bool(RULES.map_gen.item_gen_probability) {
            item_list.append(gen_dungeon_item(mid.floor), 1);
            tile.item_list = Some(item_list);
        }
    }
    
}

