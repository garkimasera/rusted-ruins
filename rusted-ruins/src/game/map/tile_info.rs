use crate::game::Game;
use common::gamedata::*;
use geom::{Direction, Vec2d};

pub fn print_tile_info(_game: &Game, _pos: Vec2d) {
    game_log_i!("tile-information-no-info");
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TileInfoQuery {
    pub move_symbol: Option<SpecialTileKind>,
    pub boundary: Option<(Direction, BoundaryBehavior)>,
}

pub fn tile_info_query(gd: &GameData, pos: Vec2d) -> TileInfoQuery {
    let map = gd.get_current_map();

    let tinfo = &map.observed_tile[pos];

    let move_symbol = match tinfo.special {
        SpecialTileKind::Stairs { .. } | SpecialTileKind::SiteSymbol { .. } => Some(tinfo.special),
        _ => None,
    };

    let boundary = if pos.0 == 0 {
        Some((Direction::W, map.boundary.w))
    } else if pos.0 == (map.w - 1) as i32 {
        Some((Direction::E, map.boundary.e))
    } else if pos.1 == 0 {
        Some((Direction::N, map.boundary.n))
    } else if pos.1 == (map.h - 1) as i32 {
        Some((Direction::S, map.boundary.s))
    } else {
        None
    };

    TileInfoQuery {
        move_symbol,
        boundary,
    }
}
