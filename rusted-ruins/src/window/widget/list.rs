
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
}

#[derive(Clone, Copy, Debug)]
pub enum ListWidgetResponse {
    Select(u32), SelectionChanged,
}

impl ListWidget {
    pub fn new<R: Into<Rect>>(rect: R, rows: ListRow, column_pos: Vec<i32>) -> ListWidget {
        Self::with_hrow(rect, rows, column_pos, 26)
    }
    
    pub fn with_hrow<R: Into<Rect>>(
        rect: R, rows: ListRow, column_pos: Vec<i32>, h_row: i32) -> ListWidget {
        
        let rect = rect.into();
        let n_row;
        let mut cache = Vec::new();
        
        match &rows {
            &ListRow::Str(ref rows) => {
                assert!(column_pos.len() == 1);
                n_row = rows.len();
                for r in rows {
                    cache.push(TextCache::new(&[r], FontKind::M, UI_CFG.color.normal_font.into()));
                }
            },
            &ListRow::IconStr(ref rows) => {
                assert!(column_pos.len() == 2);
                n_row = rows.len();
                for r in rows {
                    cache.push(TextCache::new(&[&r.1], FontKind::M, UI_CFG.color.normal_font.into()));
                }
            }
        }

        ListWidget {
            rect: rect,
            rows: rows,
            n_row: n_row,
            h_row: h_row,
            column_pos: column_pos,
            cache: cache,
            current_choice: 0,
        }
    }

    pub fn single<R: Into<Rect>>(rect: R, choices: Vec<String>) -> ListWidget {
        Self::with_hrow(rect, ListRow::Str(choices), vec![0], UI_CFG.list_widget.h_row_with_text)
    }
}

impl WidgetTrait for ListWidget {
    type Response =  ListWidgetResponse;
    fn process_command(&mut self, command: &Command) -> Option<ListWidgetResponse> {
        match *command {
            Command::Enter => {
                Some(ListWidgetResponse::Select(self.current_choice))
            },
            Command::Move { dir } => {
                match dir.vdir {
                    VDirection::Up => {
                        if self.current_choice == 0 {
                            self.current_choice = self.n_row as u32 - 1;
                        }else{
                            self.current_choice -= 1;
                        }
                        return Some(ListWidgetResponse::SelectionChanged);
                    },
                    VDirection::Down => {
                        if self.current_choice == self.n_row as u32 - 1 {
                            self.current_choice = 0;
                        }else{
                            self.current_choice += 1;
                        }
                        return Some(ListWidgetResponse::SelectionChanged);
                    }
                    _ => (),
                }
                None
            }
            _ => None
        }
    }

    fn draw(&mut self, canvas: &mut WindowCanvas, sv: &mut SdlValues) {
        
        let h_row = self.h_row;
        let left_margin = UI_CFG.list_widget.left_margin as i32;

        // Draw highlighted row background
        let highlight_rect = Rect::new(
            self.rect.x, self.rect.y + h_row * self.current_choice as i32,
            self.rect.w as u32, h_row as u32);
        canvas.set_draw_color(UI_CFG.color.window_bg_highlight.into());
        check_draw!(canvas.fill_rect(highlight_rect));

        // Draw each rows
        fn draw_text(t: &Texture, canvas: &mut WindowCanvas, rect: Rect, x: i32, y: i32) {
            let w = t.query().width;
            let h = t.query().height;

            let dest = Rect::new(rect.x + x, rect.y + y, w, h);
            check_draw!(canvas.copy(t, None, dest));
        };

        fn draw_icon(sv: &SdlValues, idx: IconIdx, canvas: &mut WindowCanvas, rect: Rect, x: i32, y: i32) {
            use common::basic::ICON_SIZE;
            let (t, orig) = sv.tex().get_icon(idx);
            let dest = Rect::new(rect.x + x, rect.y + y, ICON_SIZE, ICON_SIZE);
            check_draw!(canvas.copy(t, orig, dest));
        }
        
        match self.rows {
            ListRow::Str(_) => {
                for i in 0..self.n_row {
                    let tex = sv.tt_group(&mut self.cache[i]);
                    draw_text(&tex[0], canvas, self.rect,
                              self.column_pos[0] + left_margin, h_row * i as i32);
                }
            },
            ListRow::IconStr(ref r) => {
                for i in 0..self.n_row {
                    draw_icon(sv, r[i].0, canvas, self.rect,
                              self.column_pos[0] + left_margin, h_row * i as i32);
                    
                    let tex = sv.tt_group(&mut self.cache[i]);
                    draw_text(&tex[0], canvas, self.rect,
                              self.column_pos[1] + left_margin, h_row * i as i32);
                }
            },
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

