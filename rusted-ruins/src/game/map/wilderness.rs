use crate::game::extrait::*;
use crate::game::map::builder::MapBuilder;
use crate::game::InfoGetter;
use common::gamedata::*;
use common::gobj;
use geom::*;
use once_cell::sync::Lazy;
use regex::Regex;
use rules::biome::{BiomeDetail, SubBiomeDetail};
use rules::RULES;

pub fn generate_wilderness(gd: &GameData, pos: Vec2d) -> Option<Map> {
    let (biome, _sub_biomes) = get_biome(gd, pos)?;

    let destination = Destination::MapIdWithPos(
        gd.get_current_mapid(),
        gd.chara_pos(CharaId::Player).unwrap(),
    );
    let boundary = MapBoundary::from_same_destination(destination);

    let mut map = MapBuilder::from_map_gen_id("wilderness")
        .tile(biome.tile)
        .wall(biome.wall)
        .map_boundary(boundary)
        .build();

    // Generate plants
    for &(item_idx, weight) in &biome.plants {
        for pos in map.tile.iter_idx() {
            if !rng::gen_bool(weight / 100.0) {
                continue;
            }

            let mut item = crate::game::item::gen::gen_item_from_idx(item_idx, 1);
            item.randomize_time();
            let tile = &mut map.tile[pos];

            if tile.wall.is_empty() && tile.item_list.is_empty() {
                map.locate_item(item, pos, 1);
            }
        }
    }

    // Generate items
    for &(item_idx, weight) in &biome.items {
        for pos in map.tile.iter_idx() {
            if !rng::gen_bool(weight / 100.0) {
                continue;
            }

            let item = crate::game::item::gen::gen_item_from_idx(item_idx, 1);
            let tile = &mut map.tile[pos];

            if tile.wall.is_empty() && tile.item_list.is_empty() {
                map.locate_item(item, pos, 1);
            }
        }
    }

    // Wilderness map is revealed by default.
    map.reveal(|_| true);

    Some(map)
}

fn get_biome(
    gd: &GameData,
    pos: Vec2d,
) -> Option<(&'static BiomeDetail, Vec<&'static SubBiomeDetail>)> {
    if !gd.get_current_mapid().is_region_map() {
        return None;
    }
    let tile_info = &gd.get_current_map().tile[pos];
    let mut biome = None;
    let mut sub_biome = vec![];

    for tile_idx in tile_info.tile.0.iter().filter_map(|tile| tile.idx()) {
        let tile_id = gobj::idx_to_id(tile_idx);
        let biome_name = if let Some(biome_name) = to_biome_name(tile_id) {
            biome_name
        } else {
            continue;
        };
        if let Some(b) = RULES.biome.biomes.get(biome_name) {
            biome = Some(b);
        }
        if let Some(b) = RULES.biome.sub_biomes.get(biome_name) {
            sub_biome.push(b);
        }
    }

    if biome.is_some() {
        Some((biome.unwrap(), sub_biome))
    } else {
        None
    }
}

fn to_biome_name(name: &str) -> Option<&str> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new("rm.([a-zA-Z-_]+)-[0-9]+").unwrap());
    RE.captures(name)
        .map(|cap| cap.get(1).map(|m| m.as_str()))
        .flatten()
}
