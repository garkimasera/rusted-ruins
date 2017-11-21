
use window::{Window, DialogWindow, DialogResult};
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdlvalues::*;
use game::{Game, Animation, Command, DoPlayerAction, InfoGetter};
use config::UI_CFG;
use draw::border::draw_rect_border;
use eventhandler::InputMode;
use super::widget::*;
use common::gobj;
use common::gamedata::item::FilteredItemList;
use text;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemWindowMode {
    List, PickUp,
}

pub struct ItemWindow {
    rect: Rect,
    list: ListWidget,
    mode: ItemWindowMode,
    n_row: u32,
    current_page: u32,
    actual_idx: Vec<usize>,
}

impl ItemWindow {
    pub fn new(mode: ItemWindowMode, pa: DoPlayerAction) -> ItemWindow {
        let rect = UI_CFG.item_window.rect.into();
        
        let mut item_window = ItemWindow {
            rect: rect,
            list: ListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32), ListRow::IconStr(vec![]), vec![0, 26]),
            mode: mode,
            n_row: UI_CFG.item_window.n_row,
            current_page: 0,
            actual_idx: Vec::new(),
        };
        item_window.update_by_mode(pa);
        item_window
    }

    fn update_by_mode(&mut self, pa: DoPlayerAction) {
        let gd = pa.gd();
        
        match self.mode {
            ItemWindowMode::List => {
                use common::gamedata::chara::CharaId;
                let item_list = &gd.chara.get(CharaId::Player).item_list;
                let filtered_list = FilteredItemList::all(item_list);
                self.update_list(filtered_list);
            }
            ItemWindowMode::PickUp => {
                let item_list = gd.item_on_player_tile().unwrap();
                let filtered_list = FilteredItemList::all(item_list);
                self.update_list(filtered_list);
            }
        }
    }

    fn update_list(&mut self, list: FilteredItemList) {
        let mut rows: Vec<(IconIdx, String)> = Vec::new();
        self.actual_idx.clear();

        for (idx, item, n_item) in list.skip((self.current_page * self.n_row) as usize) {
            let item_text = format!(
                "{} x {}",
                text::obj_txt(&gobj::get_obj(item.idx).id).to_owned(),
                n_item);
            rows.push((IconIdx::Item(item.idx), item_text));
            self.actual_idx.push(idx);
        }
        self.list.set_rows(ListRow::IconStr(rows));
    }

    fn do_action_for_item(&mut self, mut pa: DoPlayerAction, i: usize) -> DialogResult {
        match self.mode {
            ItemWindowMode::List => {
                DialogResult::Continue
            }
            ItemWindowMode::PickUp => {
                pa.pick_up_item(i, 1);
                let result = if pa.gd().is_item_on_player_tile() {
                    DialogResult::Continue
                } else {
                    DialogResult::Close
                };
                self.update_by_mode(pa);
                result
            }
        }
    }
}

impl Window for ItemWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);
        self.list.draw(canvas, sv);
    }
}

impl DialogWindow for ItemWindow {
    fn process_command(&mut self, command: Command, pa: DoPlayerAction) -> DialogResult {
        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(i) => { // Any item is selected
                    let i = self.actual_idx[i as usize];
                    return self.do_action_for_item(pa, i);
                }
                _ => (),
            }
            return DialogResult::Continue;
        }
        self.list.process_command(&command);
        
        match command {
            Command::Cancel => {
                DialogResult::Close
            },
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }
}

