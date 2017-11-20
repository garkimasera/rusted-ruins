
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

pub enum ItemWindowMode {
    List, PickUp,
}

pub struct ItemWindow {
    rect: Rect,
    list: ListWidget,
    mode: ItemWindowMode,
    n_row: u32,
    current_page: u32,
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
        };
        item_window.update_by_mode(pa);
        item_window
    }

    fn update_by_mode(&mut self, pa: DoPlayerAction) {
        let gd = pa.gd();
        
        match self.mode {
            ItemWindowMode::PickUp => {
                let item_list = gd.item_on_player_tile().unwrap();
                let filtered_list = FilteredItemList::all(item_list);
                self.update_list(filtered_list);
            }
            _ => { unimplemented!() }
        }
    }

    fn update_list(&mut self, list: FilteredItemList) {
        let mut rows: Vec<(IconIdx, String)> = Vec::new();

        for (idx, item, _n_item) in list.skip((self.current_page * self.n_row) as usize) {
            let item_text = text::obj_txt(&gobj::get_obj(item.idx).id).to_owned();
            rows.push((IconIdx::Item(item.idx), item_text));
        }
        self.list.set_rows(ListRow::IconStr(rows));
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
