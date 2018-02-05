
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
    rows: ListRow,
    n_row: usize,
    h_row: i32,
    column_pos: Vec<i32>,
    cache: Vec<TextCache>,
    current_choice: u32,
}

pub enum ListRow {
    Str(Vec<String>),
    IconStr(Vec<(IconIdx, String)>),
    StrIconStr(Vec<(String, IconIdx, String)>),
}

impl ListRow {
    fn len(&self) -> usize {
        match *self {
            ListRow::Str(ref v) => { v.len() }
            ListRow::IconStr(ref v) => { v.len() }
            ListRow::StrIconStr(ref v) => { v.len() }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ListWidgetResponse {
    Select(u32), SelectionChanged,
}

impl ListWidget {
    pub fn new<R: Into<Rect>>(rect: R, rows: ListRow, column_pos: Vec<i32>) -> ListWidget {
        Self::with_hrow(rect, rows, column_pos, 26)
    }

    /// Create ListWidget with specified height per row
    pub fn with_hrow<R: Into<Rect>>(
        rect: R, rows: ListRow, column_pos: Vec<i32>, h_row: i32) -> ListWidget {
        
        let rect = rect.into();

        let mut list_widget = ListWidget {
            rect: rect,
            rows: ListRow::Str(Vec::new()),
            n_row: 0,
            h_row: h_row,
            column_pos: column_pos,
            cache: Vec::new(),
            current_choice: 0,
        };
        list_widget.set_rows(rows);
        list_widget
    }

    pub fn single<R: Into<Rect>>(rect: R, choices: Vec<String>) -> ListWidget {
        Self::with_hrow(rect, ListRow::Str(choices), vec![0], UI_CFG.list_widget.h_row_with_text)
    }

    pub fn set_rows(&mut self, rows: ListRow) {
        let n_row;
        let mut cache = Vec::new();
        
        match &rows {
            &ListRow::Str(ref rows) => {
                assert!(self.column_pos.len() == 1);
                n_row = rows.len();
                for r in rows {
                    cache.push(TextCache::new(&[r], FontKind::M, UI_CFG.color.normal_font.into()));
                }
            },
            &ListRow::IconStr(ref rows) => {
                assert!(self.column_pos.len() == 2);
                n_row = rows.len();
                for r in rows {
                    cache.push(TextCache::new(&[&r.1], FontKind::M, UI_CFG.color.normal_font.into()));
                }
            }
            &ListRow::StrIconStr(ref rows) => {
                assert!(self.column_pos.len() == 3);
                n_row = rows.len();
                for r in rows {
                    cache.push(TextCache::new(
                        &[&r.0, &r.2], FontKind::M, UI_CFG.color.normal_font.into()));
                }
            }
        }
        self.n_row = n_row;
        self.rows = rows;
        self.cache = cache;
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
        let max_w = match self.rows {
            ListRow::Str(_) => {
                let mut max_w = 0;
                for i in 0..self.n_row {
                    let tex = sv.tt_group(&mut self.cache[i]);
                    let w = tex[0].query().width;
                    if max_w < w { max_w = w }
                }
                max_w
            }
            ListRow::IconStr(_) => {
                unimplemented!()
            }
            ListRow::StrIconStr(_) => {
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

        match self.rows {
            ListRow::Str(_) => {
                for i in 0..self.n_row {
                    let h = h_row * i as i32;
                    let tex = sv.tt_group(&mut self.cache[i]);
                    draw_text(&tex[0], canvas, self.rect,
                              self.column_pos[0] + left_margin, h, self.rect.width());
                }
            }
            ListRow::IconStr(ref r) => {
                for i in 0..self.n_row {
                    let h = h_row * i as i32;
                    draw_icon(sv, r[i].0, canvas, self.rect,
                              self.column_pos[0] + left_margin, h);
                    
                    let tex = sv.tt_group(&mut self.cache[i]);
                    draw_text(&tex[0], canvas, self.rect,
                              self.column_pos[1] + left_margin, h,
                              self.rect.width() - self.column_pos[1] as u32);
                }
            }
            ListRow::StrIconStr(ref r) => {
                for i in 0..self.n_row {
                    let h = h_row * i as i32;
                    draw_icon(sv, r[i].1, canvas, self.rect,
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

