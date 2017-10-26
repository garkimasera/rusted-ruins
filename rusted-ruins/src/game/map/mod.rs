
pub mod builder;

use array2d::Vec2d;
use common::gamedata::GameData;
use common::gamedata::map::{Map, MapId};
use common::gamedata::site::DungeonKind;
use rand::{Rng, thread_rng};
use rules::RULES;
use super::chara::create_npc_chara;

pub fn gen_npcs(gd: &mut GameData, mid: MapId, n: u32, floor_level: u32) {
    for _ in 0..n {
        
        let p = choose_empty_tile(gd.site.get_map(mid));
        let race = RULES.map_gen.choose_race(DungeonKind::Cave);
        let chara = create_npc_chara(race, floor_level);
        gd.add_chara_to_map(chara, ::common::gamedata::chara::CharaKind::OnMap, mid, p);
    }
}

/// Choose one empty tile in random
pub fn choose_empty_tile(map: &Map) -> Vec2d {
    let mut rng = thread_rng();
    
    loop {
        let p = Vec2d::new(rng.gen_range(0, map.w) as i32, rng.gen_range(0, map.h) as i32);

        if map.tile[p].wall.is_none() && map.tile[p].chara.is_none() {
            return p;
        }
    }
}

