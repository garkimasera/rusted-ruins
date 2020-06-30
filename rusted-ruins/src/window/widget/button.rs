use super::WidgetTrait;
use crate::config::UI_CFG;
use crate::context::*;
use crate::game::command::{Command, MouseButton};
use sdl2::rect::Rect;

/// Simple label widget.
pub struct ButtonWidget {
    rect: Rect,
    cache: TextCache,
    covered: bool,
    pressed: bool,
}

impl ButtonWidget {
    pub fn new<R: Into<Rect>>(rect: R, s: &str, font: FontKind) -> ButtonWidget {
        let rect = rect.into();
        let cache = TextCache::one(s, font, UI_CFG.color.normal_font.into());
        ButtonWidget {
            rect,
            cache,
            covered: false,
            pressed: false,
        }
    }
}

impl WidgetTrait for ButtonWidget {
    type Response = bool;

    fn process_command(&mut self, command: &Command) -> Option<bool> {
        match command {
            Command::MouseButtonDown { x, y, button, .. } => {
                if *button == MouseButton::Left && self.rect.contains_point((*x, *y)) {
                    self.pressed = true;
                }
                None
            }
            Command::MouseButtonUp { .. } => {
                if self.pressed {
                    self.pressed = false;
                    Some(true)
                } else {
                    None
                }
            }
            Command::MouseState { x, y, .. } => {
                if self.rect.contains_point((*x, *y)) {
                    self.covered = true;
                } else {
                    self.covered = false;
                }
                None
            }
            _ => None,
        }
    }

    fn draw(&mut self, context: &mut Context) {
        let c = &UI_CFG.color;

        // Draw background and border
        let c_bg = if self.covered {
            c.button_normal_bg_covered
        } else {
            c.button_normal_bg
        };
        let c_border = if self.pressed {
            (c.button_normal_border_dark, c.button_normal_border_light)
        } else {
            (c.button_normal_border_light, c.button_normal_border_dark)
        };
        let (x, y) = (self.rect.x, self.rect.y);
        let (w, h) = (self.rect.width(), self.rect.height());
        context.fill_rect(self.rect, c_border.0);
        context.fill_rect((x + 2, y + 2, w - 2, h - 2), c_border.1);
        context.fill_rect((x + 2, y + 2, w - 4, h - 4), c_bg);

        // Draw text
        let tex = context.sv.tt_one(&mut self.cache);
        let w = tex.query().width;
        let h = tex.query().height;
        let x = self.rect.x + (self.rect.w - w as i32) / 2;
        let y = self.rect.y + (self.rect.h - h as i32) / 2;
        let dest = Rect::new(x, y, w, h);
        try_sdl!(context.canvas.copy(tex, None, dest));
    }
}
