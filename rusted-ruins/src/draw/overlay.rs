use crate::game::frequent_tex::Overlay;
use crate::game::{Game, InfoGetter};
use common::objholder::EffectImgIdx;
use common::piece_pattern::*;
use geom::*;

pub enum FogPattern {
    None,
    Fog(EffectImgIdx),
    PiecePattern(EffectImgIdx, PiecePattern),
}

pub fn view_fog(game: &Game, p: Coords) -> FogPattern {
    let view_map = &game.view_map;

    if view_map.get_tile_visible(p) {
        let mut piece_pattern_flags = PiecePatternFlags::default();
        for dir in &Direction::EIGHT_DIRS {
            piece_pattern_flags.set(*dir, view_map.get_tile_visible(p + dir.as_coords()));
        }

        let pp = piece_pattern_flags.to_piece_pattern(5);
        if pp == PiecePattern::SURROUNDED {
            FogPattern::None
        } else {
            FogPattern::PiecePattern(game.frequent_tex.overlay_idx(Overlay::Fog), pp)
        }
    } else {
        FogPattern::Fog(game.frequent_tex.overlay_idx(Overlay::Fog))
    }
}

pub fn all(game: &Game) -> Option<EffectImgIdx> {
    // If current map is indoor, don't draw night overlay
    if !game.gd.is_open_air(game.gd.get_current_mapid()) {
        return None;
    }

    let date = game.gd.time.current_date();
    let hour = date.hour;
    let minute = date.minute;
    let dawn_hour = 5;
    let dusk_hour = 18;
    assert!(dawn_hour < dusk_hour);

    if dawn_hour < hour && hour < dusk_hour {
        // Daytime
        None
    } else if hour == dawn_hour {
        Some(game.frequent_tex.overlay_idx(twilight(minute)))
    } else if hour == dusk_hour {
        Some(game.frequent_tex.overlay_idx(twilight(60 - minute)))
    } else {
        // Night
        Some(game.frequent_tex.overlay_idx(Overlay::Night))
    }
}

fn twilight(minute: u16) -> Overlay {
    if minute < 20 {
        Overlay::Twilight0
    } else if minute < 40 {
        Overlay::Twilight1
    } else {
        Overlay::Twilight2
    }
}
