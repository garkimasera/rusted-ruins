
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use config::UI_CFG;

pub fn draw_rect_border(canvas: &mut WindowCanvas, rect: Rect) {
    let light_color = UI_CFG.color.border_light;
    let dark_color = UI_CFG.color.border_dark;

    canvas.set_viewport(None);
    for n in 1..4 {
        let r = Rect::new(rect.x - n, rect.y - n, (rect.w + 2 * n) as u32, (rect.h + 2 * n) as u32);
        let c: Color = if n == 2 { light_color.into() }else{ dark_color.into() };

        canvas.set_draw_color(c);
        check_draw!(canvas.draw_rect(r));
    }
    
    canvas.set_draw_color(UI_CFG.color.window_bg.into());
    check_draw!(canvas.fill_rect(rect));
    canvas.set_viewport(rect);
}


