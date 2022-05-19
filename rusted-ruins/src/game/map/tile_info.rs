use crate::game::Game;
use common::gamedata::*;
use geom::{Coords, Direction};

pub fn print_tile_info(_game: &Game, _pos: Coords) {
    game_log!("tile-information-no-info");
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TileInfoQuery {
    pub move_symbol: Option<SpecialTileKind>,
    pub boundary: Option<(Direction, Option<Destination>)>,
    pub chara: Option<CharaId>,
}

pub fn tile_info_query(gd: &GameData, pos: Coords) -> TileInfoQuery {
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

    let chara = map.get_chara(pos);

    TileInfoQuery {
        move_symbol,
        boundary,
        chara,
    }
}
