use super::WidgetTrait;
use crate::config::UI_CFG;
use crate::context::*;
use crate::game::command::*;
use geom::*;
use sdl2::rect::Rect;

/// Simple list widget.
pub struct ListWidget<T> {
    rect: Rect,
    rows: Vec<T>,
    h_row: u32,
    /// The number of items in the current page
    n_row: u32,
    /// The number of items
    n_item: u32,
    /// The number of rows in one page
    page_size: u32,
    multiple_page: bool,
    /// x positions of each column
    column_pos: Vec<i32>,
    current_choice: u32,
    current_page: u32,
    max_page: u32,
    update_by_user: bool,
    page_label: Option<TextCache>,
    draw_border: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum ListWidgetResponse {
    Select(u32),
    SelectionChanged,
    PageChanged,
}

pub trait ListWidgetRow {
    const N_COLUMN: usize;
    fn draw(&mut self, context: &mut Context, rect: Rect, column_pos: &[i32]);
}

pub type TextListWidget = ListWidget<TextCache>;

impl<T: ListWidgetRow> ListWidget<T> {
    /// Create empty ListWidget.
    /// If update_by_user is true, user must call update_rows() after page changed.
    pub fn new<R: Into<Rect>>(
        rect: R,
        column_pos: Vec<i32>,
        page_size: u32,
        multiple_page: bool,
        update_by_user: bool,
    ) -> ListWidget<T> {
        let rect = rect.into();
        let h_row = UI_CFG.list_widget.h_row_default;

        let mut w = ListWidget {
            rect,
            rows: Vec::new(),
            h_row,
            n_row: 0,
            n_item: 0,
            page_size,
            multiple_page,
            column_pos,
            current_choice: 0,
            current_page: 0,
            max_page: 0,
            update_by_user,
            page_label: None,
            draw_border: multiple_page,
        };
        if multiple_page {
            w.update_page_label();
        }
        w
    }

    fn set_rows(&mut self, rows: Vec<T>) {
        if self.multiple_page {
            self.n_row = std::cmp::min(self.page_size, rows.len() as u32);
        } else {
            self.n_row = rows.len() as u32;
        }
        self.rows = rows;
        if self.rect.height() < self.h_row * self.n_row {
            self.rect.set_height(self.h_row * self.n_row);
        }
    }

    pub fn set_items(&mut self, items: Vec<T>) {
        self.n_item = items.len() as u32;
        self.set_rows(items);
    }

    pub fn set_n_item(&mut self, n_item: u32) {
        self.n_item = n_item;
        if self.multiple_page {
            if n_item > 0 {
                self.max_page = (n_item - 1) / self.page_size;
            } else {
                self.max_page = 0;
            }
            if self.current_page > self.max_page {
                self.set_page(self.max_page)
            } else {
                self.update_page_label();
            }
        }
    }

    pub fn set_page(&mut self, page: u32) {
        self.current_page = page;
        self.update_page_label();
    }

    // pub fn get_page(&self) -> u32 {
    //     self.current_page
    // }

    // pub fn get_max_page(&self) -> u32 {
    //     self.max_page
    // }

    pub fn page_item_idx(&self) -> (u32, u32) {
        let start = if self.multiple_page && self.update_by_user {
            self.page_size * self.current_page
        } else {
            0
        };
        let end = if self.multiple_page {
            std::cmp::min(start + self.page_size, self.n_item)
        } else {
            self.n_item
        };
        (start, end)
    }

    /// Get current choice
    /// This function considers current page position
    pub fn get_current_choice(&self) -> u32 {
        self.current_page * self.page_size + self.current_choice
    }

    pub fn update_rows_by_func<F: FnMut(u32) -> T>(&mut self, mut f: F) {
        let mut rows = Vec::new();
        let (start, end) = self.page_item_idx();
        for i in start..end {
            rows.push(f(i));
        }
        self.set_rows(rows);
    }

    /// Transmute an idx of item to an idx of row
    fn row_idx(&mut self, i: u32) -> u32 {
        if self.update_by_user {
            i - self.page_size * self.current_page
        } else {
            i
        }
    }

    fn update_page_label(&mut self) {
        let text = format!(
            "{} {}/{}",
            "page:",
            self.current_page + 1,
            self.max_page + 1
        );
        if self.multiple_page {
            self.page_label = Some(TextCache::one(
                text,
                FontKind::M,
                UI_CFG.color.normal_font.into(),
            ));
        }
    }

    fn get_idx_from_pos(&self, x: i32, y: i32) -> Option<u32> {
        if !self.rect.contains_point((x, y)) {
            return None;
        }
        let y = (y - self.rect.y) as u32;
        let idx = (y / self.h_row) as u32;
        if idx >= self.n_item {
            return None;
        }
        Some(idx)
    }
}

impl ListWidget<TextCache> {
    /// Create simple list with text only
    pub fn text_choices<R: Into<Rect>>(rect: R, choices: Vec<String>) -> ListWidget<TextCache> {
        let n_item = choices.len() as u32;
        let choices: Vec<TextCache> = choices
            .into_iter()
            .map(|s| TextCache::new(&[s], FontKind::M, UI_CFG.color.normal_font.into()))
            .collect();

        let mut list = ListWidget::new(
            rect,
            vec![UI_CFG.list_widget.left_margin],
            n_item,
            false,
            false,
        );
        list.h_row = UI_CFG.list_widget.h_row_with_text;
        list.set_rows(choices);
        list.set_n_item(n_item);
        list
    }

    /// Adjust widget size to fit inner contents
    /// Returns adjusted size
    pub fn adjust_widget_size(&mut self, sv: &mut SdlValues) -> (u32, u32) {
        let (w, h) = self.get_adjusted_widget_size(sv);
        let rect = Rect::new(self.rect.x, self.rect.y, w, h);
        self.rect = rect;
        (w, h)
    }

    /// Helper function to adjust widget size
    /// SdlValues is needed to calculate text size from text cache
    fn get_adjusted_widget_size(&mut self, sv: &mut SdlValues) -> (u32, u32) {
        let h = UI_CFG.list_widget.h_row_with_text as u32 * self.rows.len() as u32;
        let max_w = {
            let mut max_w = 0;
            for i in 0..self.n_row {
                let tex = sv.tt_one(&mut self.rows[i as usize]);
                let w = tex.query().width;
                if max_w < w {
                    max_w = w
                }
            }
            max_w
        };
        const MARGIN_FOR_BORDER: u32 = 6;
        (
            max_w + UI_CFG.list_widget.left_margin as u32 + MARGIN_FOR_BORDER,
            h,
        )
    }
}

impl<T: ListWidgetRow> WidgetTrait for ListWidget<T> {
    type Response = ListWidgetResponse;
    fn process_command(&mut self, command: &Command) -> Option<ListWidgetResponse> {
        match *command {
            Command::Enter => {
                if !self.rows.is_empty() {
                    let i = self.current_choice + self.current_page * self.page_size;
                    Some(ListWidgetResponse::Select(i))
                } else {
                    None
                }
            }
            Command::Move { dir } => {
                audio::play_sound("select-item");

                if self.n_row == 0 {
                    return None;
                }
                match dir.vdir {
                    VDirection::Up => {
                        if self.current_choice == 0 {
                            self.current_choice = self.n_row as u32 - 1;
                        } else {
                            self.current_choice -= 1;
                        }
                        return Some(ListWidgetResponse::SelectionChanged);
                    }
                    VDirection::Down => {
                        if self.current_choice == self.n_row as u32 - 1 {
                            self.current_choice = 0;
                        } else {
                            self.current_choice += 1;
                        }
                        return Some(ListWidgetResponse::SelectionChanged);
                    }
                    // Switching page
                    VDirection::None if self.multiple_page && self.max_page > 0 => {
                        let new_page = match dir.hdir {
                            HDirection::Left => {
                                if self.current_page == 0 {
                                    self.max_page
                                } else {
                                    self.current_page - 1
                                }
                            }
                            HDirection::Right => {
                                if self.current_page == self.max_page {
                                    0
                                } else {
                                    self.current_page + 1
                                }
                            }
                            _ => {
                                return None;
                            }
                        };
                        self.set_page(new_page);

                        if new_page == self.max_page {
                            let n_choice_last_page = self.n_item % self.page_size;
                            let n_choice_last_page = if n_choice_last_page == 0 {
                                self.page_size
                            } else {
                                n_choice_last_page
                            };
                            if self.current_choice >= n_choice_last_page {
                                self.current_choice = n_choice_last_page - 1;
                            }
                        }
                        return Some(ListWidgetResponse::PageChanged);
                    }
                    _ => (),
                }
                None
            }
            Command::MouseButtonUp { x, y, button } => {
                if button == MouseButton::Left {
                    if let Some(idx) = self.get_idx_from_pos(x, y) {
                        Some(ListWidgetResponse::Select(idx))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Command::MouseState { x, y } => {
                if let Some(idx) = self.get_idx_from_pos(x, y) {
                    if self.current_choice != idx {
                        self.current_choice = idx;
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn draw(&mut self, context: &mut Context) {
        // Draw page label
        if self.multiple_page {
            let tc = self.page_label.as_mut().unwrap();
            let tex = context.sv.tt_one(tc);
            let w = tex.query().width;
            let h = tex.query().height;
            let x = self.rect.right() - w as i32;
            let y = (self.h_row * self.page_size) as i32;
            let dest = Rect::new(x, y, w, h);
            try_sdl!(context.canvas.copy(tex, None, dest));
        }

        // Draw borders between rows
        if self.draw_border {
            let a = if self.multiple_page { 1 } else { 0 };
            for i in 1..(self.page_size + a) {
                let y = (self.h_row * i) as i32;
                context.canvas.set_draw_color(UI_CFG.color.list_border);
                try_sdl!(context.canvas.draw_line((0, y), (self.rect.right(), y)));
            }
        }

        if self.n_row == 0 {
            return;
        }

        let h_row = self.h_row;

        // Draw highlighted row background
        let highlight_rect = Rect::new(
            self.rect.x,
            self.rect.y + h_row as i32 * self.current_choice as i32,
            self.rect.w as u32,
            h_row as u32,
        );
        context
            .canvas
            .set_draw_color(UI_CFG.color.window_bg_highlight);
        try_sdl!(context.canvas.fill_rect(highlight_rect));

        // Draw each rows
        let (start, end) = self.page_item_idx();
        for (j, i) in (start..end).enumerate() {
            let i = self.row_idx(i);
            if let Some(row) = self.rows.get_mut(i as usize) {
                let rect = Rect::new(
                    self.rect.x,
                    self.rect.y + j as i32 * self.h_row as i32,
                    self.rect.width(),
                    self.h_row,
                );
                row.draw(context, rect, &self.column_pos);
            }
        }

        let canvas = &mut context.canvas;

        // Draw highlight row borders
        canvas.set_draw_color(UI_CFG.color.border_highlight_dark);
        try_sdl!(canvas.draw_rect(highlight_rect));
        let r = Rect::new(
            highlight_rect.x + 1,
            highlight_rect.y + 1,
            highlight_rect.w as u32 - 2,
            highlight_rect.h as u32 - 2,
        );
        canvas.set_draw_color(UI_CFG.color.border_highlight_light);
        try_sdl!(canvas.draw_rect(r));
    }
}

impl ListWidgetRow for TextCache {
    const N_COLUMN: usize = 1;

    fn draw(&mut self, context: &mut Context, rect: Rect, column_pos: &[i32]) {
        let tex = context.sv.tt_one(self);
        let w = tex.query().width;
        let h = tex.query().height;
        let w = std::cmp::min(w, (rect.w - column_pos[0]) as u32);
        let dest = Rect::new(rect.x + column_pos[0], rect.y, w, h);
        try_sdl!(context.canvas.copy(tex, None, dest));
    }
}

impl ListWidgetRow for IconIdx {
    const N_COLUMN: usize = 1;

    fn draw(&mut self, context: &mut Context, rect: Rect, column_pos: &[i32]) {
        let (t, orig) = context.sv.tex().get_icon(*self);
        let dest = Rect::new(rect.x + column_pos[0], rect.y, orig.width(), orig.height());
        try_sdl!(context.canvas.copy(t, orig, dest));
    }
}

/// N_COLUMN of T1 and T2 must be 1
impl<T1: ListWidgetRow, T2: ListWidgetRow> ListWidgetRow for (T1, T2) {
    const N_COLUMN: usize = 2;

    fn draw(&mut self, context: &mut Context, rect: Rect, column_pos: &[i32]) {
        let w = column_pos[1] - column_pos[0];
        let rect0 = Rect::new(rect.x + column_pos[0], rect.y, w as u32, rect.height());
        self.0.draw(context, rect0, &[0]);
        let w = rect.right() - column_pos[1];
        let rect1 = Rect::new(rect.x + column_pos[1], rect.y, w as u32, rect.height());
        self.1.draw(context, rect1, &[0]);
    }
}

/// N_COLUMN of T1, T2 and T3 must be 1
impl<T1: ListWidgetRow, T2: ListWidgetRow, T3: ListWidgetRow> ListWidgetRow for (T1, T2, T3) {
    const N_COLUMN: usize = 3;

    fn draw(&mut self, context: &mut Context, rect: Rect, column_pos: &[i32]) {
        let w = column_pos[1] - column_pos[0];
        let rect0 = Rect::new(rect.x + column_pos[0], rect.y, w as u32, rect.height());
        self.0.draw(context, rect0, &[0]);
        let w = column_pos[2] - column_pos[1];
        let rect1 = Rect::new(rect.x + column_pos[1], rect.y, w as u32, rect.height());
        self.1.draw(context, rect1, &[0]);
        let w = rect.right() - column_pos[2];
        let rect2 = Rect::new(rect.x + column_pos[2], rect.y, w as u32, rect.height());
        self.2.draw(context, rect2, &[0]);
    }
}
