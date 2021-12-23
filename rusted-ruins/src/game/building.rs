use super::Game;
use common::gamedata::*;
use common::gobj;
use common::obj::*;
use common::objholder::*;
use geom::*;

pub fn start_build(game: &mut Game, pos: Vec2d, builder: CharaId, build_obj: BuildObj) {
    let needed_materials = match &build_obj {
        BuildObj::Tile(id) => {
            let tile_idx: TileIdx = gobj::id_to_idx(id);
            let tile_obj = gobj::get_obj(tile_idx);
            &tile_obj.materials
        }
        BuildObj::Wall(id) => {
            let wall_idx: WallIdx = gobj::id_to_idx(id);
            let wall_obj = gobj::get_obj(wall_idx);
            &wall_obj.materials
        }
    };

    if !is_buildable(&game.gd, pos) {
        return;
    }

    let item_list = game
        .gd
        .get_item_list_mut(ItemListLocation::Chara { cid: builder });

    // Check player has needed materials
    for &(ref item_id, n) in needed_materials {
        let item_idx: ItemIdx = gobj::id_to_idx(item_id);
        let has = item_list.count(item_idx);
        if has < n {
            let needed = n - has;
            let item = crate::text::obj_txt(item_id);
            game_log!("building-shortage-material"; item=item, n=needed);
            return;
        }
    }

    // Consume needed materials
    for &(ref item_id, n) in needed_materials {
        let item_idx: ItemIdx = gobj::id_to_idx(item_id);
        item_list.consume(item_idx, n, |_, _| {}, false);
    }

    finish_build(game, pos, &build_obj);
}

pub fn finish_build(game: &mut Game, pos: Vec2d, build_obj: &BuildObj) {
    match build_obj {
        BuildObj::Tile(id) => {
            let tile_idx: TileIdx = gobj::id_to_idx(id);
            let map = game.gd.get_current_map_mut();
            map.set_tile(pos, tile_idx, None);
        }
        BuildObj::Wall(id) => {
            let wall_idx: WallIdx = gobj::id_to_idx(id);
            let map = game.gd.get_current_map_mut();
            map.set_wall(pos, wall_idx);
        }
    }
    audio::play_sound("finish-build");
}

fn is_buildable(gd: &GameData, pos: Vec2d) -> bool {
    let map = gd.get_current_map();

    if !map.is_inside(pos) {
        return false;
    }

    if map.tile[pos].wall.is_empty() {
        let tile = gobj::get_obj(map.tile[pos].main_tile());
        match tile.kind {
            TileKind::Ground => true,
            TileKind::Water => false,
        }
    } else {
        false
    }
}

/// Returns buildable object and its needed skill level list
pub fn build_obj_list() -> Vec<(BuildObj, u32)> {
    let mut list = Vec::new();

    for (i, tile) in gobj::get_objholder().tile.iter().enumerate() {
        if let Some(skill_level) = tile.build_skill {
            let id = gobj::idx_to_id(TileIdx::from_usize(i));
            list.push((BuildObj::Tile(id.into()), skill_level));
        }
    }

    for (i, wall) in gobj::get_objholder().wall.iter().enumerate() {
        if let Some(skill_level) = wall.build_skill {
            let id = gobj::idx_to_id(WallIdx::from_usize(i));
            list.push((BuildObj::Wall(id.into()), skill_level));
        }
    }

    list
}
