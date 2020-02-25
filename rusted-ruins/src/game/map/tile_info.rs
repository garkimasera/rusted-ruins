use crate::game::Game;
use common::gamedata::{BoundaryBehavior, Map};
use geom::{Direction, Vec2d};

pub fn print_tile_info(game: &Game, tile: Vec2d) {
    game_log_i!("tile-information-no-info");
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TileInfoQuery {
    pub boundary: Option<(Direction, BoundaryBehavior)>,
}

pub fn tile_info_query(map: &Map, tile: Vec2d) -> TileInfoQuery {
    let boundary = if tile.0 == 0 {
        Some((Direction::W, map.boundary.w))
    } else if tile.0 == (map.w - 1) as i32 {
        Some((Direction::E, map.boundary.e))
    } else if tile.1 == 0 {
        Some((Direction::N, map.boundary.n))
    } else if tile.1 == (map.h - 1) as i32 {
        Some((Direction::S, map.boundary.s))
    } else {
        None
    };
    TileInfoQuery { boundary }
}
