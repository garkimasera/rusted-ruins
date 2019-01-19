//! This module provides functions for auto generated dungeons

use array2d::*;
use rng;
use common::gamedata::*;
use common::objholder::*;
use common::gobj;
use crate::game::map::builder::MapBuilder;
use crate::game::saveload::gen_box_id;
use rules::RULES;

/// Add a new dungeon
pub fn add_dungeon_site(gd: &mut GameData, dungeon_kind: DungeonKind, pos: Vec2d) -> SiteId {
    let floor_range = &RULES.dungeon_gen[&dungeon_kind].floor_range;
    let mut site = Site::new(rng::gen_range(floor_range[0], floor_range[1]));
    site.content = SiteContent::AutoGenDungeon { dungeon_kind };
    gd.add_site(site, SiteKind::AutoGenDungeon, RegionId::default(), pos).unwrap()
}

/// Extend dungion site by one floor
pub fn extend_site_floor(gd: &mut GameData, sid: SiteId) {
    let floor = gd.region.get_site(sid).floor_num();
    let is_deepest_floor = floor >= gd.region.get_site(sid).max_floor() - 1;
    let map = match gd.region.get_site(sid).content {
        SiteContent::AutoGenDungeon { dungeon_kind } => {
            let map_size = RULES.dungeon_gen[&dungeon_kind].map_size;
            let tile_idx = gobj::id_to_idx(&RULES.dungeon_gen[&dungeon_kind].terrain[0][0]);
            let wall_idx = gobj::id_to_idx(&RULES.dungeon_gen[&dungeon_kind].terrain[0][1]);
            MapBuilder::new(map_size.0 as u32, map_size.1 as u32)
                .floor(floor)
                .tile(tile_idx)
                .wall(wall_idx)
                .deepest_floor(is_deepest_floor)
                .build()
        }
        _ => {
            MapBuilder::new(40, 40).floor(floor).build()
        }
    };

    let map_random_id = gen_box_id(gd);
    let mid = gd.add_map(map, sid, map_random_id);
    super::map::gen_npcs(gd, mid, 10, mid.floor());
    super::map::gen_items(gd, mid);
    
    if is_deepest_floor {
        add_for_deepest_floor(gd, mid);
    }
}

/// Add items for deepest floor of dungeon
pub fn add_for_deepest_floor(gd: &mut GameData, mid: MapId) {
    let map = gd.region.get_map_mut(mid);

    let p = if let Some(p) = crate::game::map::choose_empty_tile(map) { p } else { return; };

    let idx: ItemIdx = gobj::id_to_idx("ancient-box");
    let item_obj: &ItemObject = gobj::get_obj(idx);
    let item = Item {
        idx,
        flags: item_obj.default_flags,
        kind: item_obj.kind,
        rank: ItemRank::default(),
        attributes: vec![],
    };

    let mut item_list = ItemList::new();
    item_list.append(item, 1);
    map.tile[p].item_list = Some(item_list);
}

