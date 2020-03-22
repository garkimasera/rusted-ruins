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
    knob_space_rect: Rect,
    knob_rect: Rect,
    up_button_hover: bool,
    down_button_hover: bool,
    page_size: u32,
    total_size: u32,
    value: u32,
    limit: u32,
}

pub enum ScrollResponse {
    Scrolled,
}

impl VScrollWidget {
    pub fn new(rect: Rect, page_size: u32) -> VScrollWidget {
        let cfg = &UI_CFG.vscroll_widget;
        let up_button_rect = Rect::new(rect.x, rect.y, cfg.width, cfg.button_height);
        let down_button_rect = Rect::new(
            rect.x,
            rect.bottom() as i32 - cfg.button_height as i32,
            cfg.width,
            cfg.button_height,
        );
        let knob_space_rect = Rect::new(
            rect.x + 1,
            rect.y + cfg.button_height as i32 + 1,
            cfg.width - 2,
            rect.height() - cfg.button_height * 2 - 2,
        );

        VScrollWidget {
            rect,
            up_button_rect,
            down_button_rect,
            knob_space_rect,
            knob_rect: knob_space_rect,
            up_button_hover: false,
            down_button_hover: false,
            page_size,
            total_size: 0,
            value: 0,
            limit: 0,
        }
    }

    pub fn set_total_size(&mut self, total_size: u32) {
        self.total_size = total_size;
        if total_size <= self.page_size {
            self.limit = 0;
        } else {
            self.limit = total_size - self.page_size;
        }
        if self.limit < self.value {
            self.value = self.limit;
        }
        self.update_knob();
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn page_size(&self) -> u32 {
        self.page_size
    }

    pub fn total_size(&self) -> u32 {
        self.total_size
    }

    fn try_up_scroll(&mut self) -> Option<ScrollResponse> {
        if self.value > 0 {
            self.value -= 1;
            Some(ScrollResponse::Scrolled)
        } else {
            None
        }
    }

    fn try_down_scroll(&mut self) -> Option<ScrollResponse> {
        if self.value < self.limit {
            self.value += 1;
            Some(ScrollResponse::Scrolled)
        } else {
            None
        }
    }

    fn update_knob(&mut self) {
        let knob_size = if self.page_size < self.total_size {
            self.knob_space_rect.height() * self.page_size / self.total_size
        } else {
            self.knob_space_rect.height()
        };
        self.knob_rect.set_height(std::cmp::max(
            UI_CFG.vscroll_widget.min_knob_size,
            knob_size,
        ));
        self.knob_rect.y = ((self.knob_space_rect.height() - self.knob_rect.height()) * self.value
            / self.limit) as i32
            + self.knob_space_rect.y;
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
            Command::MouseButtonDown { x, y, .. } => {
                if self.up_button_rect.contains_point((*x, *y)) && self.value > 0 {
                    self.try_up_scroll()
                } else if self.down_button_rect.contains_point((*x, *y)) && self.value < self.limit
                {
                    self.try_down_scroll()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn draw(&mut self, context: &mut Context) {
        let cfg = &UI_CFG.vscroll_widget;
        let color = &UI_CFG.color;
        let knob_space_outer_rect = Rect::new(
            self.rect.x,
            self.rect.y + cfg.button_height as i32,
            self.rect.width(),
            self.rect.height() - cfg.button_height * 2,
        );
        context.draw_rect(knob_space_outer_rect, color.vscroll_border.into());
        context.draw_rect(self.knob_space_rect, color.vscroll_border_inner.into());

        // Draw arrow buttons
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

        // Draw knob
        context.draw_rect(self.knob_rect, color.vscroll_knob_border.into());
    }
}
