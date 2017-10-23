
pub mod builder;

use array2d::Vec2d;
use common::gamedata::GameData;
use common::gamedata::map::{Map, MapId};
use rand::{Rng, thread_rng};

pub fn gen_npcs(gd: &mut GameData, mid: MapId, n: u32) {
    for _ in 0..n {
        
        let p = choose_empty_tile(gd.site.get_map(mid));
        let mut chara = ::common::gamedata::chara::Chara::default();
        chara.params.spd = 100;
        chara.params.str = 20;
        chara.rel = ::common::gamedata::chara::Relationship::HOSTILE;
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

