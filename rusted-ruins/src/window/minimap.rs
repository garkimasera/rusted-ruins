use crate::config::SCREEN_CFG;
use crate::context::*;
use crate::game::Game;
use crate::game::{Animation, InfoGetter};
use crate::window::Window;
use common::gobj;
use geom::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct MiniMapWindow {
    rect: Rect,
}

impl MiniMapWindow {
    pub fn new() -> MiniMapWindow {
        MiniMapWindow {
            rect: SCREEN_CFG.minimap_window.into(),
        }
    }
}

impl Window for MiniMapWindow {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game<'_>,
        _anim: Option<(&Animation, u32)>,
    ) {
        context.canvas.set_draw_color((0, 0, 0));
        context.set_viewport(None);
        try_sdl!(context.canvas.fill_rect(self.rect));
        context.set_viewport(self.rect);
        draw_minimap(context.canvas, self.rect, game, context.sv);
    }
}

const RECT_SIZE: u32 = 3;
const RECT_SIZE_I: i32 = RECT_SIZE as i32;

fn draw_minimap(
    canvas: &mut WindowCanvas,
    rect: Rect,
    game: &Game<'_>,
    _sv: &mut SdlValues<'_, '_>,
) {
    use std::cmp::{max, min};
    let map = game.gd.get_current_map();
    let map_size = map.size();
    let n_width = (rect.width() / RECT_SIZE) as i32;
    let n_height = (rect.height() / RECT_SIZE) as i32;
    let center_p = game.gd.player_pos();
    let top_left = (center_p.0 - n_width / 2, center_p.1 - n_height / 2);
    let bottom_right = (
        min(map_size.0 as i32 - 1, center_p.0 + n_width / 2),
        min(map_size.1 as i32 - 1, center_p.1 + n_height / 2),
    );
    let (dx, dy) = (top_left.0 * RECT_SIZE_I, top_left.1 * RECT_SIZE_I);
    let top_left = (max(0, top_left.0), max(0, top_left.1));

    for p in RectIter::new(top_left, bottom_right) {
        let color = if p == center_p {
            (255, 255, 0)
        } else if let Some(wall_idx) = map.observed_tile[p].wall.idx() {
            gobj::get_obj(wall_idx).symbol_color
        } else if map.observed_tile[p].tile {
            gobj::get_obj(map.tile[p].main_tile()).symbol_color
        } else {
            continue;
        };
        let color = Color::RGB(color.0, color.1, color.2);

        let draw_rect = Rect::new(
            p.0 * RECT_SIZE_I - dx,
            p.1 * RECT_SIZE_I - dy,
            RECT_SIZE,
            RECT_SIZE,
        );
        canvas.set_draw_color(color);
        try_sdl!(canvas.fill_rect(draw_rect));
    }
}
