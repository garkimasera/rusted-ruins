use super::Game;
use common::gamedata::*;
use common::gobj;
use common::obj::*;
use common::objholder::*;
use geom::*;

pub fn start_build(game: &mut Game<'_>, pos: Vec2d, builder: CharaId) {
    let wall_id = "wooden-wall-01";
    let wall_idx: WallIdx = gobj::id_to_idx(wall_id);
    let wall_obj = gobj::get_obj(wall_idx);

    if !is_buildable(&game.gd, pos) {
        return;
    }

    let item_list = game
        .gd
        .get_item_list_mut(ItemListLocation::Chara { cid: builder });

    let materials = wall_obj.materials.as_ref().unwrap();

    // Check player has needed materials
    for &(ref item_id, n) in materials {
        let item_idx: ItemIdx = gobj::id_to_idx(item_id);
        let has = item_list.count(item_idx);
        if has < n {
            let needed = n - has;
            let item = crate::text::obj_txt(item_id);
            game_log_i!("building-shortage-material"; item=item, n=needed);
            return;
        }
    }

    // Consume needed materials
    for &(ref item_id, n) in materials {
        let item_idx: ItemIdx = gobj::id_to_idx(item_id);
        item_list.consume(item_idx, n, |_, _| {}, false);
    }

    finish_build(game, pos, wall_idx);
}

pub fn finish_build(game: &mut Game<'_>, pos: Vec2d, wall_idx: WallIdx) {
    let map = game.gd.get_current_map_mut();
    map.set_wall(pos, wall_idx);
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
