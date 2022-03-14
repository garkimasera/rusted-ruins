use super::{MovableWidget, WidgetTrait};
use crate::config::UI_CFG;
use crate::context::*;
use crate::game::command::{Command, MouseButton};
use common::gobj;
use common::objholder::UiImgIdx;
use sdl2::rect::Rect;

/// Simple label widget.
pub struct CloseButtonWidget {
    rect: Rect,
    icon: IconIdx,
    covered: bool,
    pressed: bool,
}

pub enum CloseButtonIconKind {
    Close,
    #[allow(unused)]
    Return,
}

impl CloseButtonWidget {
    pub fn new<R: Into<Rect>>(rect: R, icon: CloseButtonIconKind) -> Self {
        let icon_id = match icon {
            CloseButtonIconKind::Close => "!icon-close",
            CloseButtonIconKind::Return => "!icon-return",
        };
        let idx: UiImgIdx = gobj::id_to_idx(icon_id);

        Self {
            rect: rect.into(),
            icon: IconIdx::UiImg { idx, i_pattern: 0 },
            covered: false,
            pressed: false,
        }
    }

    pub fn from_bottom_right(x: i32, y: i32, icon: CloseButtonIconKind) -> Self {
        let w = UI_CFG.close_button_widget.w;
        let h = UI_CFG.close_button_widget.h;
        let rect = Rect::new(x - w as i32, y - h as i32, w, h);
        Self::new(rect, icon)
    }
}

impl MovableWidget for CloseButtonWidget {
    fn move_to(&mut self, x: i32, y: i32) {
        self.rect.x = x;
        self.rect.y = y;
    }
}

impl WidgetTrait for CloseButtonWidget {
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

    fn draw(&mut self, context: &mut Context<'_, '_, '_, '_>) {
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

        // Draw icon
        let (tex, orig) = context.sv.tex().get_icon(self.icon);
        let w = orig.width();
        let h = orig.height();
        let x = self.rect.x + (self.rect.w - w as i32) / 2;
        let y = self.rect.y + (self.rect.h - h as i32) / 2;
        let dest = Rect::new(x, y, w, h);
        try_sdl!(context.canvas.copy(tex, None, dest));
    }
}
