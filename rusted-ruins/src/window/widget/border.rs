
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use crate::context::*;
use crate::config::UI_CFG;
use super::WidgetTrait;

pub struct HBorder {
    rect: Rect,
    light: Color,
    dark: Color,
}

impl HBorder {
    pub fn new(start: (i32, i32), len: u32) -> HBorder {
        let rect = Rect::new(start.0, start.1, len, 3);
        
        HBorder {
            rect,
            light: UI_CFG.color.border_light.into(),
            dark: UI_CFG.color.border_dark.into(),
        }
    }
}

impl WidgetTrait for HBorder {
    type Response = ();

    fn draw(&mut self, context: &mut Context) {
        let canvas = &mut context.canvas;
        canvas.set_viewport(None);
        canvas.set_draw_color(self.light);
        check_draw!(canvas.draw_line(
            (self.rect.x, self.rect.y + 1), (self.rect.x + self.rect.w, self.rect.y + 1)));
        canvas.set_draw_color(self.dark);
        check_draw!(canvas.draw_line(
            (self.rect.x, self.rect.y), (self.rect.x + self.rect.w, self.rect.y)));
        check_draw!(canvas.draw_line(
            (self.rect.x, self.rect.y + 2), (self.rect.x + self.rect.w, self.rect.y + 2)));
    }
}

pub struct VBorder {
    rect: Rect,
    light: Color,
    dark: Color,
}

impl VBorder {
    pub fn new(start: (i32, i32), len: u32) -> VBorder {
        let rect = Rect::new(start.0, start.1, 3, len);
        
        VBorder {
            rect,
            light: UI_CFG.color.border_light.into(),
            dark: UI_CFG.color.border_dark.into(),
        }
    }
}

impl WidgetTrait for VBorder {
    type Response = ();

    fn draw(&mut self, context: &mut Context) {
        let canvas = &mut context.canvas;
        canvas.set_viewport(None);
        canvas.set_draw_color(self.light);
        check_draw!(canvas.draw_line(
            (self.rect.x + 1, self.rect.y), (self.rect.x + 1, self.rect.y + self.rect.h)));
        canvas.set_draw_color(self.dark);
        check_draw!(canvas.draw_line(
            (self.rect.x, self.rect.y), (self.rect.x, self.rect.y + self.rect.h)));
        check_draw!(canvas.draw_line(
            (self.rect.x + 2, self.rect.y), (self.rect.x + 2, self.rect.y + self.rect.h)));
    }
}

