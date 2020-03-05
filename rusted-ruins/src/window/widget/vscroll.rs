use super::WidgetTrait;
use crate::config::UI_CFG;
use crate::context::*;
use crate::game::command::*;
use common::gobj;
use common::objholder::UIImgIdx;
use sdl2::rect::Rect;

/// Vertical scroll widget
pub struct VScrollWidget {
    rect: Rect,
    up_button_rect: Rect,
    down_button_rect: Rect,
    up_button_hover: bool,
    down_button_hover: bool,
}

pub enum ScrollResponse {}

impl VScrollWidget {
    pub fn new(rect: Rect) -> VScrollWidget {
        let cfg = &UI_CFG.vscroll_widget;
        let up_button_rect = Rect::new(rect.x, rect.y, cfg.width, cfg.button_height);
        let down_button_rect = Rect::new(
            rect.x,
            rect.bottom() as i32 - cfg.button_height as i32,
            cfg.width,
            cfg.button_height,
        );
        VScrollWidget {
            rect,
            up_button_rect,
            down_button_rect,
            up_button_hover: false,
            down_button_hover: false,
        }
    }
}

impl WidgetTrait for VScrollWidget {
    type Response = ScrollResponse;

    fn process_command(&mut self, command: &Command) -> Option<ScrollResponse> {
        match command {
            Command::MouseState { x, y, .. } => {
                self.up_button_hover = self.up_button_rect.contains_point((*x, *y));
                self.down_button_hover = self.down_button_rect.contains_point((*x, *y));
                None
            }
            _ => None,
        }
    }

    fn draw(&mut self, context: &mut Context) {
        let cfg = &UI_CFG.vscroll_widget;
        let color = &UI_CFG.color;
        let middle_bar_rect = Rect::new(
            self.rect.x,
            self.rect.y + cfg.button_height as i32,
            self.rect.width(),
            self.rect.height() - cfg.button_height * 2,
        );
        context.draw_rect(middle_bar_rect, color.vscroll_border.into());
        let middle_bar_inner_rect = Rect::new(
            middle_bar_rect.x + 1,
            middle_bar_rect.y + 1,
            middle_bar_rect.width() - 2,
            middle_bar_rect.height() - 2,
        );
        context.draw_rect(middle_bar_inner_rect, color.vscroll_border_inner.into());

        lazy_static! {
            static ref VSCROLL_BUTTON: UIImgIdx = gobj::id_to_idx("!vscroll-button");
        };

        context.render_tex_n(
            *VSCROLL_BUTTON,
            self.up_button_rect,
            if self.up_button_hover { 1 } else { 0 },
        );
        context.render_tex_n(
            *VSCROLL_BUTTON,
            self.down_button_rect,
            if self.down_button_hover { 3 } else { 2 },
        );
    }
}
