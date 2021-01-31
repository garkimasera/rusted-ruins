use crate::game::map::builder::MapBuilder;
use crate::game::InfoGetter;
use common::gamedata::*;
use common::gobj;
use geom::*;
use regex::Regex;
use rules::biome::{BiomeDetail, SubBiomeDetail};
use rules::RULES;

pub fn generate_wilderness(gd: &GameData, pos: Vec2d) -> Option<Map> {
    let (biome, _sub_biomes) = if let Some(b) = get_biome(gd, pos) {
        b
    } else {
        return None;
    };

    let destination = Destination::MapIdWithPos(
        gd.get_current_mapid(),
        gd.chara_pos(CharaId::Player).unwrap(),
    );
    let boundary = MapBoundary::from_same_destination(destination);

    let map = MapBuilder::from_map_gen_id("wilderness")
        .tile(biome.tile)
        .wall(biome.wall)
        .map_boundary(boundary)
        .build();

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

    for tile_idx in tile_info.tile.iter().filter_map(|tile| tile.idx()) {
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
    lazy_static! {
        static ref RE: Regex = Regex::new("rm.([a-zA-Z-_]+)-[0-9]+").unwrap();
    }
    RE.captures(name)
        .map(|cap| cap.get(1).map(|m| m.as_str()))
        .flatten()
}
