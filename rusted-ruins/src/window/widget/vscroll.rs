use super::WidgetTrait;
use crate::config::UI_CFG;
use crate::context::*;
use common::gobj;
use common::objholder::UIImgIdx;
use sdl2::rect::Rect;

/// Vertical scroll widget
pub struct VScrollWidget {
    rect: Rect,
}

impl VScrollWidget {
    pub fn new(rect: Rect) -> VScrollWidget {
        VScrollWidget { rect }
    }
}

impl WidgetTrait for VScrollWidget {
    type Response = ();

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
        let up_button_rect = Rect::new(self.rect.x, self.rect.y, cfg.width, cfg.button_height);
        context.render_tex_n(*VSCROLL_BUTTON, up_button_rect, 0);
        let down_button_rect = Rect::new(
            self.rect.x,
            self.rect.bottom() as i32 - cfg.button_height as i32,
            cfg.width,
            cfg.button_height,
        );
        context.render_tex_n(*VSCROLL_BUTTON, down_button_rect, 2);
    }
}
