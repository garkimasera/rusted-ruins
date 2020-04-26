use super::Game;
use common::gamedata::*;
use common::gobj;
use common::obj::*;
use common::objholder::*;
use common::piece_pattern::{PiecePatternFlags, WallIdxPP};
use geom::*;

pub fn start_build(game: &mut Game, pos: Vec2d, _builder: CharaId) {
    let wall_id = "wooden-wall-01";
    let wall_idx: WallIdx = gobj::id_to_idx(wall_id);
    let _wall_obj = gobj::get_obj(wall_idx);

    if !is_buildable(&game.gd, pos) {
        return;
    }

    finish_build(game, pos, wall_idx);
}

pub fn finish_build(game: &mut Game, pos: Vec2d, wall_idx: WallIdx) {
    let map = game.gd.get_current_map_mut();
    let wall_obj = gobj::get_obj(wall_idx);
    map.tile[pos].wall = WallIdxPP::new(wall_idx);

    for p in RectIter::new(pos + Direction::NW.as_vec(), pos + Direction::SE.as_vec()) {
        if map.tile[p].wall.idx() != Some(wall_idx) {
            continue;
        }
        let ppf = PiecePatternFlags::from_fn(p, |p| {
            if map.is_inside(p) {
                map.tile[p].wall.idx() == Some(wall_idx)
            } else {
                false
            }
        });
        let wallpp =
            WallIdxPP::with_piece_pattern(wall_idx, ppf.to_piece_pattern(wall_obj.img.n_pattern));
        map.tile[p].wall = wallpp;
    }
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
