//! This module provides functions for auto generated dungeons

use crate::game::map::builder::MapBuilder;
use crate::game::saveload::gen_box_id;
use common::gamedata::*;
use common::gobj;
use common::objholder::*;
use geom::*;
use rng::{self, GameRng, SliceRandom};
use rules::dungeon_gen::DungeonGenParams;
use rules::RULES;

/// Add a new dungeon
pub fn add_dungeon_site(gd: &mut GameData, dungeon_kind: DungeonKind, pos: Vec2d) -> SiteId {
    let floor_range = &RULES.dungeon_gen[&dungeon_kind].floor_range;
    let n_floor = rng::gen_range(floor_range[0]..floor_range[1]);
    let mut site = Site::new(n_floor, None);
    site.content = SiteContent::AutoGenDungeon { dungeon_kind };
    gd.add_site(
        site,
        SiteKind::AutoGenDungeon,
        RegionId::default(),
        Some(pos),
    )
    .unwrap()
}

/// Extend dungion site by one floor
pub fn extend_site_floor(gd: &mut GameData, sid: SiteId) {
    let floor = gd.region.get_site(sid).floor_num();
    let is_deepest_floor = floor >= gd.region.get_site(sid).max_floor() - 1;
    let map = match gd.region.get_site(sid).content {
        SiteContent::AutoGenDungeon { dungeon_kind } => {
            let rule = &RULES.dungeon_gen[&dungeon_kind];
            let gen_params = &RULES.dungeon_gen[&dungeon_kind];
            let floor_gen_id = &gen_params
                .floor_gen
                .choose_weighted(&mut GameRng, |item| item.1)
                .unwrap()
                .0;
            let tile_idx = gobj::id_to_idx(&rule.terrain[0][0]);
            let wall_idx = gobj::id_to_idx(&rule.terrain[0][1]);
            let mut map = MapBuilder::from_map_gen_id(floor_gen_id)
                .floor(floor)
                .tile(tile_idx)
                .wall(wall_idx)
                .deepest_floor(is_deepest_floor)
                .music(&gen_params.music)
                .build();
            set_sub_walls(&mut map, rule);
            map
        }
        _ => MapBuilder::new(40, 40).floor(floor).build(),
    };

    let map_random_id = gen_box_id(gd);
    let mid = gd.add_map(map, sid, map_random_id);
    super::map::gen_npcs(gd, mid, 10, mid.floor());
    super::map::gen_items(gd, mid);

    if is_deepest_floor {
        add_for_deepest_floor(gd, mid);
    }
}

pub fn set_sub_walls(map: &mut Map, rule: &DungeonGenParams) {
    for (sub_wall_id, weight) in &rule.sub_walls {
        let wall_idx: WallIdx = if let Some(wall_idx) = gobj::id_to_idx_checked(sub_wall_id) {
            wall_idx
        } else {
            error!("unknown wall id {}", sub_wall_id);
            return;
        };

        let binary_map = map_generator::binary::create_binary_fractal(map.size().into(), *weight);

        for p in binary_map.iter_idx() {
            let tile = &mut map.tile[p];
            if !tile.wall.is_empty() && binary_map[p] {
                tile.wall.set_idx(wall_idx);
            }
        }
    }
}

/// Add items for deepest floor of dungeon
pub fn add_for_deepest_floor(gd: &mut GameData, mid: MapId) {
    let map = gd.region.get_map_mut(mid);

    let p = if let Some(p) = crate::game::map::choose_empty_tile(map) {
        p
    } else {
        return;
    };

    let idx: ItemIdx = gobj::id_to_idx("ancient-box");
    let item_obj: &ItemObject = gobj::get_obj(idx);
    let item = Item {
        idx,
        flags: item_obj.default_flags,
        kind: item_obj.kind,
        quality: ItemQuality::default(),
        attributes: vec![],
    };

    let mut item_list = ItemList::new();
    item_list.append(item, 1);
    map.tile[p].item_list = item_list;
}
