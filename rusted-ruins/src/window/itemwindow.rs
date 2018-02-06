
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
use common::gamedata::item::{FilteredItemList, ItemListLocation, ItemFilter, ItemLocation};
use text;

pub type ActionCallback = FnMut(&mut DoPlayerAction, ItemLocation) -> DialogResult;
pub enum ItemWindowMode {
    List, PickUp,
    Select {
        ill: ItemListLocation,
        filter: ItemFilter,
        action: Box<ActionCallback>,
    }
}

pub struct ItemWindow {
    rect: Rect,
    list: ListWidget,
    mode: ItemWindowMode,
    n_row: u32,
    item_locations: Vec<ItemLocation>,
}

impl ItemWindow {
    pub fn new(mode: ItemWindowMode, pa: &mut DoPlayerAction) -> ItemWindow {
        let rect = UI_CFG.item_window.rect.into();
        
        let mut item_window = ItemWindow {
            rect: rect,
            list: ListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32), ListRowKind::IconStr, vec![0, 26],
                Some(UI_CFG.item_window.n_row), 26),
            mode: mode,
            n_row: UI_CFG.item_window.n_row,
            item_locations: Vec::new(),
        };
        item_window.update_by_mode(pa);
        item_window
    }

    pub fn new_select(ill: ItemListLocation, filter: ItemFilter,
                  action: Box<ActionCallback>, pa: &mut DoPlayerAction) -> ItemWindow {
        let mode = ItemWindowMode::Select {
            ill, filter, action
        };
        ItemWindow::new(mode, pa)
    }

    fn update_by_mode(&mut self, pa: &mut DoPlayerAction) {
        let gd = pa.gd();
        
        match self.mode {
            ItemWindowMode::List => {
                let ill = ItemListLocation::Chara { cid: ::common::gamedata::chara::CharaId::Player };
                let filtered_list = gd.get_filtered_item_list(ill, ItemFilter::all());
                self.update_list(filtered_list);
            }
            ItemWindowMode::PickUp => {
                let ill = ItemListLocation::OnMap {
                    mid: gd.get_current_mapid(),
                    pos: gd.player_pos(),
                };
                let filtered_list = gd.get_filtered_item_list(ill, ItemFilter::all());
                self.update_list(filtered_list);
            }
            ItemWindowMode::Select { ill, filter, ..} => {
                let filtered_list = gd.get_filtered_item_list(ill, filter);
                self.update_list(filtered_list);
            }
        }
    }

    fn update_list(&mut self, list: FilteredItemList) {
        let item_locations = &mut self.item_locations;
        self.list.update_rows_by_func(|start, page_size| {
            let mut rows = Vec::new();
            item_locations.clear();

            for (item_location, item, n_item) in list.skip(start as usize).take(page_size as usize) {
                let item_text = format!(
                    "{} x {}",
                    text::obj_txt(&gobj::get_obj(item.idx).id).to_owned(),
                    n_item);
                rows.push(ListRow::IconStr(IconIdx::Item(item.idx), item_text));
                item_locations.push(item_location);
            }
            rows
        });
    }

    fn do_action_for_item(&mut self, pa: &mut DoPlayerAction, il: ItemLocation) -> DialogResult {
        match self.mode {
            ItemWindowMode::List => {
                DialogResult::Continue
            }
            ItemWindowMode::PickUp => {
                pa.pick_up_item(il, 1);
                let result = if pa.gd().is_item_on_player_tile() {
                    self.update_by_mode(pa);
                    DialogResult::Continue
                } else {
                    DialogResult::Close
                };
                result
            }
            ItemWindowMode::Select { ref mut action, .. } => {
                action(pa, il)
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
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(i) => { // Any item is selected
                    let il = self.item_locations[i as usize];
                    return self.do_action_for_item(pa, il);
                }
                _ => (),
            }
            return DialogResult::Continue;
        }
        self.list.process_command(&command);
        
        match *command {
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

