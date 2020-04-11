use super::WidgetTrait;
use crate::config::UI_CFG;
use crate::context::*;
use crate::game::command::*;
use common::gobj;
use common::objholder::UIImgIdx;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

/// Vertical scroll widget
pub struct VScrollWidget {
    rect: Rect,
    up_button_rect: Rect,
    down_button_rect: Rect,
    knob_space_rect: Rect,
    knob_rect: Rect,
    up_button_hover: bool,
    down_button_hover: bool,
    up_button_last: Option<Instant>,
    down_button_last: Option<Instant>,
    gripped: Option<(i32, u32)>,
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
            up_button_last: None,
            down_button_last: None,
            gripped: None,
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

    fn try_move_to(&mut self, new_value: u32) -> Option<ScrollResponse> {
        if new_value == self.value || new_value > self.limit {
            None
        } else {
            self.value = new_value;
            Some(ScrollResponse::Scrolled)
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
        self.knob_rect.y = if self.limit > 0 {
            ((self.knob_space_rect.height() - self.knob_rect.height()) * self.value / self.limit)
                as i32
        } else {
            0
        } + self.knob_space_rect.y;
    }
}

impl WidgetTrait for VScrollWidget {
    type Response = ScrollResponse;

    fn process_command(&mut self, command: &Command) -> Option<ScrollResponse> {
        match command {
            Command::MouseState { x, y, .. } => {
                if let Some((origin_y, origin_value)) = self.gripped.as_ref() {
                    let diff = *y - origin_y;
                    let knob_free_space_size =
                        self.knob_space_rect.height() - self.knob_rect.height();
                    if knob_free_space_size == 0 {
                        return None;
                    }
                    let diff = diff * self.limit as i32 / knob_free_space_size as i32;
                    let new_value = *origin_value as i32 + diff;
                    let new_value = if new_value < 0 { 0 } else { new_value as u32 };
                    return self.try_move_to(new_value);
                }

                let button_repeat_duration =
                    Duration::from_millis(UI_CFG.vscroll_widget.button_repeat_duration);
                self.up_button_hover = self.up_button_rect.contains_point((*x, *y));
                if self.up_button_hover {
                    if let Some(up_button_last) = self.up_button_last.as_mut() {
                        if Instant::now().duration_since(*up_button_last) > button_repeat_duration {
                            return self.try_up_scroll();
                        }
                    }
                }
                self.down_button_hover = self.down_button_rect.contains_point((*x, *y));
                if self.down_button_hover {
                    if let Some(down_button_last) = self.down_button_last.as_mut() {
                        if Instant::now().duration_since(*down_button_last) > button_repeat_duration
                        {
                            return self.try_down_scroll();
                        }
                    }
                }
                None
            }
            Command::MouseButtonDown { x, y, button, .. } => {
                let (x, y) = (*x, *y);
                if *button != MouseButton::Left {
                    return None;
                }
                if self.up_button_rect.contains_point((x, y)) && self.value > 0 {
                    self.up_button_last = Some(Instant::now());
                    self.try_up_scroll()
                } else if self.down_button_rect.contains_point((x, y)) && self.value < self.limit {
                    self.down_button_last = Some(Instant::now());
                    self.try_down_scroll()
                } else if self.knob_rect.contains_point((x, y)) {
                    self.gripped = Some((y, self.value));
                    None
                } else if self.knob_space_rect.contains_point((x, y)) {
                    if y < self.knob_rect.top() {
                        self.try_move_to(0)
                    } else if y > self.knob_rect.bottom() {
                        self.try_move_to(self.limit)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Command::MouseButtonUp { button, .. } => {
                if *button != MouseButton::Left {
                    return None;
                }
                self.up_button_last = None;
                self.down_button_last = None;
                self.gripped = None;
                None
            }
            Command::MouseWheel { y, .. } => {
                if self.gripped.is_some() {
                    return None;
                }
                if *y > 0 {
                    self.try_up_scroll()
                } else if *y < 0 {
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
        context.draw_rect(knob_space_outer_rect, color.vscroll_border);
        context.draw_rect(self.knob_space_rect, color.vscroll_border_inner);

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
        context.fill_rect(self.knob_rect, color.vscroll_knob);
        context.draw_rect(self.knob_rect, color.vscroll_knob_border);
    }
}
