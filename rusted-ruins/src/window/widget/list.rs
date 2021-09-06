use super::{ScrollResponse, VScrollWidget, WidgetTrait};
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
    /// x positions of each column
    column_pos: Vec<i32>,
    current_choice: u32,
    /// Need update after Scrolled response returned or not
    update_by_user: bool,
    draw_border: bool,
    scroll: Option<VScrollWidget>,
    header: Option<T>,
}

#[derive(Clone, Copy, Debug)]
pub enum ListWidgetResponse {
    Select(u32),
    SelectForMenu(u32),
    SelectionChanged,
    Scrolled,
}

pub trait ListWidgetRow {
    const N_COLUMN: usize;
    fn row_draw(&mut self, context: &mut Context<'_, '_, '_, '_>, rect: Rect, column_pos: &[i32]);
}

pub type TextListWidget = ListWidget<TextCache>;

impl<T: ListWidgetRow> ListWidget<T> {
    /// Create empty ListWidget.
    /// If update_by_user is true, user must call update_rows() after page changed.
    pub fn new<R: Into<Rect>>(
        rect: R,
        column_pos: Vec<i32>,
        page_size: u32,
        update_by_user: bool,
    ) -> ListWidget<T> {
        let rect = rect.into();
        let h_row = UI_CFG.list_widget.h_row_default;

        ListWidget {
            rect,
            rows: Vec::new(),
            h_row,
            n_row: 0,
            n_item: 0,
            page_size,
            column_pos,
            current_choice: 0,
            update_by_user,
            draw_border: false,
            scroll: None,
            header: None,
        }
    }

    pub fn with_scroll_bar<R: Into<Rect>>(
        rect: R,
        column_pos: Vec<i32>,
        page_size: u32,
        update_by_user: bool,
    ) -> ListWidget<T> {
        Self::with_scroll_bar_inner(rect, column_pos, page_size, update_by_user, None)
    }

    pub fn with_header<R: Into<Rect>>(
        rect: R,
        column_pos: Vec<i32>,
        page_size: u32,
        update_by_user: bool,
        header: T,
    ) -> ListWidget<T> {
        Self::with_scroll_bar_inner(rect, column_pos, page_size, update_by_user, Some(header))
    }

    fn with_scroll_bar_inner<R: Into<Rect>>(
        rect: R,
        column_pos: Vec<i32>,
        page_size: u32,
        update_by_user: bool,
        header: Option<T>,
    ) -> ListWidget<T> {
        let rect = rect.into();
        let h_row = UI_CFG.list_widget.h_row_default;
        let y_start = if header.is_some() { h_row as i32 } else { 0 };

        let scroll_w = UI_CFG.vscroll_widget.width;
        let rect = Rect::new(rect.x, rect.y, rect.width() - scroll_w - 1, rect.height());
        let vscroll_rect = Rect::new(
            rect.right() + 1,
            rect.y + y_start,
            scroll_w,
            rect.height() - y_start as u32,
        );

        let scroll = VScrollWidget::new(vscroll_rect, page_size);

        ListWidget {
            rect,
            rows: Vec::new(),
            h_row,
            n_row: 0,
            n_item: 0,
            page_size,
            column_pos,
            current_choice: 0,
            update_by_user,
            draw_border: false,
            scroll: Some(scroll),
            header,
        }
    }

    fn set_rows(&mut self, rows: Vec<T>) {
        self.n_row = rows.len() as u32;
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
        if let Some(scroll) = self.scroll.as_mut() {
            scroll.set_total_size(self.n_item);
        }
    }

    pub fn page_item_idx(&self) -> (u32, u32) {
        (0, self.n_item)
    }

    /// Get current choice
    /// This function considers current scroll position
    pub fn get_current_choice(&self) -> u32 {
        if let Some(scroll) = self.scroll.as_ref() {
            scroll.value() + self.current_choice
        } else {
            self.current_choice
        }
    }

    pub fn update_rows_by_func<F: FnMut(u32) -> T>(&mut self, mut f: F) {
        let mut rows = Vec::new();
        let (start, end) = if let Some(scroll) = self.scroll.as_ref() {
            (
                scroll.value(),
                std::cmp::min(scroll.value() + scroll.page_size(), scroll.total_size()),
            )
        } else {
            (0, self.n_item)
        };
        for i in start..end {
            rows.push(f(i));
        }
        self.set_rows(rows);
    }

    /// Transmute an idx of item to an idx of row
    fn row_idx(&mut self, i: u32) -> u32 {
        i
    }

    fn get_idx_from_pos(&self, x: i32, y: i32) -> Option<u32> {
        if !self.rect.contains_point((x, y)) {
            return None;
        }
        let y_start = if self.header.is_some() {
            self.h_row as i32
        } else {
            0
        };
        let y = (y - self.rect.y - y_start) as u32;
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

        let mut list = ListWidget::new(rect, vec![UI_CFG.list_widget.left_margin], n_item, false);
        list.h_row = UI_CFG.list_widget.h_row_with_text;
        list.set_rows(choices);
        list.set_n_item(n_item);
        list
    }

    /// Adjust widget size to fit inner contents
    /// Returns adjusted size
    pub fn adjust_widget_size(&mut self, sv: &mut SdlValues<'_, '_>) -> (u32, u32) {
        let (w, h) = self.get_adjusted_widget_size(sv);
        let rect = Rect::new(self.rect.x, self.rect.y, w, h);
        self.rect = rect;
        (w, h)
    }

    /// Helper function to adjust widget size
    /// SdlValues is needed to calculate text size from text cache
    fn get_adjusted_widget_size(&mut self, sv: &mut SdlValues<'_, '_>) -> (u32, u32) {
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
        if let Some(scroll) = self.scroll.as_mut() {
            if let Some(ScrollResponse::Scrolled) = scroll.process_command(command) {
                if self.update_by_user {
                    return Some(ListWidgetResponse::Scrolled);
                } else {
                    todo!();
                }
            }
        }

        match *command {
            Command::Enter => {
                if !self.rows.is_empty() {
                    Some(ListWidgetResponse::Select(self.get_current_choice()))
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
                    _ => (),
                }
                None
            }
            Command::MouseButtonUp { x, y, button, .. } => {
                if button == MouseButton::Left || button == MouseButton::Right {
                    if let Some(idx) = self.get_idx_from_pos(x, y) {
                        let i = if let Some(scroll) = self.scroll.as_ref() {
                            idx + scroll.value()
                        } else {
                            idx
                        };
                        if button == MouseButton::Left {
                            Some(ListWidgetResponse::Select(i))
                        } else {
                            Some(ListWidgetResponse::SelectForMenu(i))
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Command::MouseState { x, y, .. } => {
                if let Some(idx) = self.get_idx_from_pos(x, y) {
                    if self.current_choice != idx {
                        self.current_choice = idx;
                        audio::play_sound("select-item");
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn draw(&mut self, context: &mut Context<'_, '_, '_, '_>) {
        let y_start = if self.header.is_some() {
            self.h_row as i32
        } else {
            0
        };

        if let Some(header) = self.header.as_mut() {
            let rect = Rect::new(self.rect.x, self.rect.y, self.rect.width(), self.h_row);
            header.row_draw(context, rect, &self.column_pos);
            context
                .canvas
                .set_draw_color(UI_CFG.color.list_header_border);
            let y = self.rect.y + self.h_row as i32 - 1;
            try_sdl!(context
                .canvas
                .draw_line((self.rect.x + 5, y), (self.rect.right() - 5, y)));
        }

        // Draw borders between rows
        if self.draw_border {
            for i in 1..self.page_size {
                let y = (self.h_row * i) as i32 + y_start;
                context.canvas.set_draw_color(UI_CFG.color.list_border);
                try_sdl!(context.canvas.draw_line((0, y), (self.rect.right(), y)));
            }
        }

        if self.n_row > 0 {
            let h_row = self.h_row;

            // Draw highlighted row background
            let highlight_rect = Rect::new(
                self.rect.x,
                self.rect.y + h_row as i32 * self.current_choice as i32 + y_start,
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
                        self.rect.y + j as i32 * self.h_row as i32 + y_start,
                        self.rect.width(),
                        self.h_row,
                    );
                    row.row_draw(context, rect, &self.column_pos);
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

        // Draw scrollbar
        if let Some(scroll) = self.scroll.as_mut() {
            scroll.draw(context);
        }
    }
}

impl ListWidgetRow for TextCache {
    const N_COLUMN: usize = 1;

    fn row_draw(&mut self, context: &mut Context<'_, '_, '_, '_>, rect: Rect, column_pos: &[i32]) {
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

    fn row_draw(&mut self, context: &mut Context<'_, '_, '_, '_>, rect: Rect, column_pos: &[i32]) {
        let h_row = UI_CFG.list_widget.h_row_default as i32;
        let icon_column_w = UI_CFG.list_widget.icon_column_w as i32;

        let (t, orig) = context.sv.tex().get_icon(*self);
        const ICON_SIZE_LIMIT: u32 = 32;
        let (w, h) = (orig.width(), orig.height());
        let (w, h) = if w > ICON_SIZE_LIMIT || h > ICON_SIZE_LIMIT {
            (w / 2, h / 2)
        } else {
            (w, h)
        };
        let x = (icon_column_w - w as i32) / 2 + column_pos[0];
        let y = (h_row - h as i32) / 2;
        let dest = Rect::new(rect.x + x, rect.y + y, w, h);
        try_sdl!(context.canvas.copy(t, orig, dest));
    }
}

impl<T> ListWidgetRow for T
where
    T: super::MovableWidget,
{
    const N_COLUMN: usize = 1;

    fn row_draw(&mut self, context: &mut Context<'_, '_, '_, '_>, rect: Rect, column_pos: &[i32]) {
        self.move_to(rect.x + column_pos[0], rect.y);
        self.draw(context);
    }
}

/// N_COLUMN of T1 and T2 must be 1
impl<T1: ListWidgetRow, T2: ListWidgetRow> ListWidgetRow for (T1, T2) {
    const N_COLUMN: usize = 2;

    fn row_draw(&mut self, context: &mut Context<'_, '_, '_, '_>, rect: Rect, column_pos: &[i32]) {
        let w = column_pos[1] - column_pos[0];
        let rect0 = Rect::new(rect.x + column_pos[0], rect.y, w as u32, rect.height());
        self.0.row_draw(context, rect0, &[0]);
        let w = rect.right() - column_pos[1];
        let rect1 = Rect::new(rect.x + column_pos[1], rect.y, w as u32, rect.height());
        self.1.row_draw(context, rect1, &[0]);
    }
}

/// N_COLUMN of T1, T2 and T3 must be 1
impl<T1: ListWidgetRow, T2: ListWidgetRow, T3: ListWidgetRow> ListWidgetRow for (T1, T2, T3) {
    const N_COLUMN: usize = 3;

    fn row_draw(&mut self, context: &mut Context<'_, '_, '_, '_>, rect: Rect, column_pos: &[i32]) {
        let w = column_pos[1] - column_pos[0];
        let rect0 = Rect::new(rect.x + column_pos[0], rect.y, w as u32, rect.height());
        self.0.row_draw(context, rect0, &[0]);
        let w = column_pos[2] - column_pos[1];
        let rect1 = Rect::new(rect.x + column_pos[1], rect.y, w as u32, rect.height());
        self.1.row_draw(context, rect1, &[0]);
        let w = rect.right() - column_pos[2];
        let rect2 = Rect::new(rect.x + column_pos[2], rect.y, w as u32, rect.height());
        self.2.row_draw(context, rect2, &[0]);
    }
}
