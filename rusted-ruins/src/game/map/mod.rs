
pub mod builder;
pub mod from_template;

use array2d::Vec2d;
use common::basic::MAX_ITEM_FOR_DRAW;
use common::gamedata::GameData;
use common::gamedata::map::{Map, MapId};
use common::gamedata::chara::CharaId;
use common::gamedata::site::{SiteContent, DungeonKind};
use common::gamedata::item::ItemList;
use super::Game;
use super::chara::CharaEx;
use super::chara::creation::create_npc_chara;
use super::item::gen::gen_dungeon_item;
use rules::RULES;

/// Switch current map to the specified map
pub fn switch_map(game: &mut Game, mid: MapId) {
    {
        let gd = &mut game.gd;
        
        trace!("Switch map to {:?}", mid);
        // If next_mid floor doesn't exist, create new floor
        if !mid.is_region_map() && gd.region.get_map_checked(mid).is_none() {
            info!("{:?} is not exist, so try to create new floor", mid);
            super::site::extend_site_floor(gd, mid.sid());
        }
        let prev_mid = gd.get_current_mapid();
        gd.set_current_mapid(mid);

        let new_player_pos = if mid.is_region_map() && !prev_mid.is_region_map()
            && mid.rid() == prev_mid.rid() { // Exit from a site to region map
                gd.region.get_site_pos(prev_mid.sid())
            } else { // Move to another floor of the same site
                let current_map = gd.get_current_map();
                if let Some(p) =
                    current_map.search_stairs(prev_mid.floor()) { p } else { current_map.entrance }
            };
    
        gd.get_current_map_mut().locate_chara(CharaId::Player, new_player_pos);
    }
    super::view::update_view_map(game);
}

pub fn gen_npcs(gd: &mut GameData, mid: MapId, n: u32, floor_level: u32) {
    let dungeon_kind = match gd.region.get_site(mid.sid()).content {
        SiteContent::AutoGenDungeon { dungeon_kind } => dungeon_kind,
        _ => DungeonKind::Ruin,
    };
    
    for _ in 0..n {
        if let Some(p) = choose_empty_tile(gd.region.get_map(mid)) {
            let chara = create_npc_chara(dungeon_kind, floor_level);
            trace!("Generate new npc {}", chara.get_name());
            gd.add_chara_to_map(chara, ::common::gamedata::chara::CharaKind::OnMap, mid, p);
        } else {
            warn!("Failed npc generating because empty tile not found");
            return;
        }
    }
}

/// Choose one empty tile in random
pub fn choose_empty_tile(map: &Map) -> Option<Vec2d> {
    use rng::gen_range;
    const MAX_TRY: usize = 10;
    
    for _ in 0..MAX_TRY {
        let p = Vec2d::new(gen_range(0, map.w) as i32, gen_range(0, map.h) as i32);
        let tile = &map.tile[p];

        // Empty tile don't has wall, chara, and isn't special tile.
        if tile.wall.is_none() && tile.chara.is_none() && tile.special.is_none() {
            return Some(p);
        }
    }

    // If random tile choosing is failed many times, count empty tiles and choose
    let n_empty_tile = map.tile.iter().filter(|t| t.wall.is_none() && t.chara.is_none()).count();
    if n_empty_tile == 0 {
        None
    } else {
        
        let r = gen_range(0, n_empty_tile);
        let p = map.tile
            .iter_with_idx()
            .filter(|&(_, t)| t.wall.is_none() && t.chara.is_none() && t.special.is_none() )
            .skip(r)
            .next()
            .unwrap()
            .0;
        Some(p)
    }
}

/// Locate some items for a new map
pub fn gen_items(gd: &mut GameData, mid: MapId) {
    use rng::*;
    let item_gen_probability = {
        let site = gd.region.get_site(mid.sid());
        match site.content {
            SiteContent::AutoGenDungeon { dungeon_kind } => {
                RULES.dungeon_gen[&dungeon_kind].item_gen_probability
            }
            _ => 0,
        }
    };
    let map = gd.region.get_map_mut(mid);

    for p in map.tile.iter_idx() {
        let tile = &mut map.tile[p];
        if tile.wall.is_some() { continue; } // Skip tile with wall

        let mut item_list = ItemList::new(10);

        if get_rng().gen_weighted_bool(item_gen_probability) {
            item_list.append(gen_dungeon_item(mid.floor()), 1);
            tile.item_list = Some(item_list);
        }
    }
    
}

pub fn update_observed_map(game: &mut Game) {
    let view_map = &game.view_map;
    let map = game.gd.get_current_map_mut();

    for p in map.tile.iter_idx() {
        if !view_map.get_tile_visible(p) { continue; }

        let tile = &map.tile[p];
        let observed_tile = &mut map.observed_tile[p];

        observed_tile.tile = Some(tile.tile);
        observed_tile.wall = tile.wall;
        observed_tile.deco = tile.deco;
        observed_tile.special = tile.special;
        observed_tile.n_item = 0;

        if let Some(ref item_list) = tile.item_list {
            for (i, &(ref item, _)) in item_list.iter().take(MAX_ITEM_FOR_DRAW).enumerate() {
                observed_tile.items[i] = item.idx;
                observed_tile.n_item += 1;
            }
        }
    }
}

