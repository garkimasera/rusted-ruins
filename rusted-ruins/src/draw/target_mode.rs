use super::mainwin::{MainWinDrawer, TargetModeDrawInfo};
use crate::context::*;
use crate::game::Game;
use common::gobj;
use common::objholder::UIImgIdx;
use geom::*;
use once_cell::sync::Lazy;

static TILE_RANGE_HIGHLIBHT: Lazy<UIImgIdx> =
    Lazy::new(|| gobj::id_to_idx("!tile-range-highlight"));

impl MainWinDrawer {
    pub fn draw_target_mode(
        &self,
        context: &mut Context,
        game: &Game,
        target_mode: &TargetModeDrawInfo,
    ) {
        let map = game.gd.get_current_map();

        for p in self.tile_range() {
            if !map.is_inside(p) {
                continue;
            }
            self.draw_target_mode_to_tile(context, p, game, target_mode);
        }
    }

    fn draw_target_mode_to_tile(
        &self,
        context: &mut Context,
        pos: Vec2d,
        game: &Game,
        target_mode: &TargetModeDrawInfo,
    ) {
        if target_mode.range.is_inside(pos) {
            if !game.view_map.get_tile_visible(pos) {
                return;
            }
            let tile_rect = self.tile_rect(pos, 0, 0);
            context.render_tex(*TILE_RANGE_HIGHLIBHT, tile_rect);
        }
    }
}
