use crate::config::UI_CFG;
use crate::context::Context;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub fn draw_window_border<R: Into<Rect>>(context: &mut Context<'_, '_, '_, '_>, rect: R) {
    let rect = rect.into();
    let light_color = UI_CFG.color.border_light;
    let dark_color = UI_CFG.color.border_dark;

    let canvas = &mut context.canvas;
    canvas.set_viewport(None);
    for n in 1..4 {
        let r = Rect::new(
            rect.x - n,
            rect.y - n,
            (rect.w + 2 * n) as u32,
            (rect.h + 2 * n) as u32,
        );
        let c: Color = if n == 2 {
            light_color.into()
        } else {
            dark_color.into()
        };

        canvas.set_draw_color(c);
        try_sdl!(canvas.draw_rect(r));
    }

    canvas.set_draw_color(UI_CFG.color.window_bg);
    try_sdl!(canvas.fill_rect(rect));
    canvas.set_viewport(rect);
}
