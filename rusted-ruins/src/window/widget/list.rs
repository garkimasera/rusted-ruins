
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};
use array2d::*;
use sdlvalues::*;
use config::UI_CFG;
use game::Command;
use super::WidgetTrait;

/// Simple list widget.
pub struct ListWidget {
    rect: Rect,
    kind: ListRowKind,
    rows: Vec<ListRow>,
    h_row: i32,
    n_row: u32,
    n_item: u32,
    page_size: Option<u32>,
    column_pos: Vec<i32>,
    cache: Vec<TextCache>,
    current_choice: u32,
    current_page: u32,
    max_page: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ListRowKind {
    Str, IconStr, StrIconStr,
}

pub enum ListRow {
    Str(String),
    IconStr(IconIdx, String),
    StrIconStr(String, IconIdx, String),
}

impl ListRow {
    fn kind(&self) -> ListRowKind {
        match *self {
            ListRow::Str(_) => ListRowKind::Str,
            ListRow::IconStr(_, _) => ListRowKind::IconStr,
            ListRow::StrIconStr(_, _, _) => ListRowKind::StrIconStr,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ListWidgetResponse {
    Select(u32), SelectionChanged, PageChanged,
}

impl ListWidget {
    /// Create empty ListWidget
    pub fn new<R: Into<Rect>>(
        rect: R, kind: ListRowKind, column_pos: Vec<i32>,
        page_size: Option<u32>, h_row: i32) -> ListWidget {
        
        let rect = rect.into();

        assert!((kind == ListRowKind::Str && column_pos.len() == 1) ||
                (kind == ListRowKind::IconStr && column_pos.len() == 2) ||
                (kind == ListRowKind::StrIconStr && column_pos.len() == 3));

        ListWidget {
            rect: rect,
            kind: kind,
            rows: Vec::new(),
            h_row: h_row,
            n_row: 0,
            n_item: 0,
            page_size: page_size,
            column_pos: column_pos,
            cache: Vec::new(),
            current_choice: 0,
            current_page: 0,
            max_page: 1,
        }
    }

    /// Create simple list with text only
    pub fn texts_choices<R: Into<Rect>>(rect: R, choices: Vec<String>) -> ListWidget {
        let n_item = choices.len() as u32;
        let rows: Vec<ListRow> = choices.into_iter().map(|s| ListRow::Str(s)).collect();

        let mut list = ListWidget::new(rect, ListRowKind::Str, vec![0],
                                       None, UI_CFG.list_widget.h_row_with_text);
        list.set_rows(rows);
        list.n_item = n_item;
        list
    }

    /// Set row data directly
    pub fn set_rows(&mut self, rows: Vec<ListRow>) {
        let mut cache = Vec::new();
        self.n_row = rows.len() as u32;

        for row in &rows {
            assert!(row.kind() == self.kind);
            match *row {
                ListRow::Str(ref s) => {
                    cache.push(TextCache::new(&[s], FontKind::M, UI_CFG.color.normal_font.into()));
                },
                ListRow::IconStr(i, ref s) => {
                    cache.push(TextCache::new(&[s], FontKind::M, UI_CFG.color.normal_font.into()));
                }
                ListRow::StrIconStr(ref s0, i, ref s1) => {
                    cache.push(TextCache::new(
                        &[s0, s1], FontKind::M, UI_CFG.color.normal_font.into()));
                }
            }
        }
        
        self.rows = rows;
        self.cache = cache;
    }

    /// Update rows for this page
    pub fn update_rows_by_func<F>(&mut self, f: F) where F: FnOnce(u32, u32) -> Vec<ListRow> {
        let page_size = self.page_size.unwrap_or(self.n_item);
        let rows = f(page_size * self.current_page, page_size);
        self.set_rows(rows);
    }

    pub fn set_n_item(&mut self, n_item: u32) {
        self.n_item = n_item;
        if let Some(page_size) = self.page_size {
            self.max_page = n_item / page_size;
        }
    }

    pub fn set_page(&mut self, page: u32) {
        self.current_page = page;
    }

    pub fn get_page(&mut self) -> u32 {
        self.current_page
    }

    /// Adjust widget size to fit inner contents
    /// Returns adjusted size
    pub fn adjust_widget_size(&mut self, sv: &mut SdlValues) -> (u32, u32) {
        let (w, h) = self.get_adjusted_widget_size(sv);
        let rect = Rect::new(self.rect.x, self.rect.y, w, h);
        self.rect = rect;
        (w, h)
    }

    /// Helper function to get widget size
    /// SdlValues is needed to calculate text size from text cache
    pub fn get_adjusted_widget_size(&mut self, sv: &mut SdlValues) -> (u32, u32) {
        let h = UI_CFG.list_widget.h_row_with_text as u32 * self.rows.len() as u32;
        let max_w = match self.kind {
            ListRowKind::Str => {
                let mut max_w = 0;
                for i in 0..self.n_row {
                    let tex = sv.tt_group(&mut self.cache[i as usize]);
                    let w = tex[0].query().width;
                    if max_w < w { max_w = w }
                }
                max_w
            }
            ListRowKind::IconStr => {
                unimplemented!()
            }
            ListRowKind::StrIconStr => {
                unimplemented!()
            }
        };
        const MARGIN_FOR_BORDER: u32 = 6;
        (max_w + UI_CFG.list_widget.left_margin as u32 + MARGIN_FOR_BORDER, h)
    }
}

impl WidgetTrait for ListWidget {
    type Response =  ListWidgetResponse;
    fn process_command(&mut self, command: &Command) -> Option<ListWidgetResponse> {
        match *command {
            Command::Enter => {
                if self.rows.len() > 0 {
                    Some(ListWidgetResponse::Select(self.current_choice))
                } else {
                    None
                }
            }
            Command::Move { dir } => {
                if self.n_row == 0 { return None; }
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
                    VDirection::None if self.page_size.is_some() && self.max_page > 0 => {
                        match dir.hdir {
                            HDirection::Left => {
                                let new_page = if self.current_page == 0 {
                                    self.max_page
                                } else {
                                    self.current_page - 1
                                };
                                self.set_page(new_page);
                                return Some(ListWidgetResponse::PageChanged);
                            }
                            HDirection::Right => {
                                let new_page = if self.current_page == self.max_page {
                                    0
                                } else {
                                    self.current_page + 1
                                };
                                self.set_page(new_page);
                                return Some(ListWidgetResponse::PageChanged);
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
                None
            }
            _ => None,
        }
    }

    fn draw(&mut self, canvas: &mut WindowCanvas, sv: &mut SdlValues) {
        if self.n_row == 0 { return; }
        
        let h_row = self.h_row;
        let left_margin = UI_CFG.list_widget.left_margin as i32;

        // Draw highlighted row background
        let highlight_rect = Rect::new(
            self.rect.x, self.rect.y + h_row * self.current_choice as i32,
            self.rect.w as u32, h_row as u32);
        canvas.set_draw_color(UI_CFG.color.window_bg_highlight.into());
        check_draw!(canvas.fill_rect(highlight_rect));

        // Draw each rows
        fn draw_text(t: &Texture, canvas: &mut WindowCanvas, rect: Rect, x: i32, y: i32,
                     max_w: u32) {
            let w = t.query().width;
            let h = t.query().height;
            let w = if w > max_w { max_w } else { w };

            let dest = Rect::new(rect.x + x, rect.y + y, w, h);
            check_draw!(canvas.copy(t, None, dest));
        };

        fn draw_icon(sv: &SdlValues, idx: IconIdx, canvas: &mut WindowCanvas, rect: Rect, x: i32, y: i32) {
            use common::basic::ICON_SIZE;
            let (t, orig) = sv.tex().get_icon(idx);
            let dest = Rect::new(rect.x + x, rect.y + y, orig.width(), orig.height());
            check_draw!(canvas.copy(t, orig, dest));
        }

        for (i, row) in self.rows.iter().enumerate() {
            match *row {
                ListRow::Str(_) => {
                    let h = h_row * i as i32;
                    let tex = sv.tt_group(&mut self.cache[i]);
                    draw_text(&tex[0], canvas, self.rect,
                              self.column_pos[0] + left_margin, h, self.rect.width());
                }
                ListRow::IconStr(icon_idx, _) => {
                    let h = h_row * i as i32;
                    draw_icon(sv, icon_idx, canvas, self.rect,
                              self.column_pos[0] + left_margin, h);
                    
                    let tex = sv.tt_group(&mut self.cache[i]);
                    draw_text(&tex[0], canvas, self.rect,
                              self.column_pos[1] + left_margin, h,
                              self.rect.width() - self.column_pos[1] as u32);
                }
                ListRow::StrIconStr(_, icon_idx, _) => {
                    let h = h_row * i as i32;
                    draw_icon(sv, icon_idx, canvas, self.rect,
                              self.column_pos[1] + left_margin, h);
                    
                    let tex = sv.tt_group(&mut self.cache[i]);
                    draw_text(&tex[0], canvas, self.rect,
                              self.column_pos[0] + left_margin, h,
                              self.column_pos[1] as u32);
                    draw_text(&tex[1], canvas, self.rect,
                              self.column_pos[2] + left_margin, h,
                              self.rect.width() - self.column_pos[2] as u32);
                }
            }
        }

        // Draw highlight row borders
        canvas.set_draw_color(UI_CFG.color.border_highlight_dark.into());
        check_draw!(canvas.draw_rect(highlight_rect));
        let r = Rect::new(highlight_rect.x + 1, highlight_rect.y + 1,
                          highlight_rect.w as u32 - 2, highlight_rect.h as u32 - 2);
        canvas.set_draw_color(UI_CFG.color.border_highlight_light.into());
        check_draw!(canvas.draw_rect(r));
    }
}

