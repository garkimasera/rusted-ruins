
use window::{Window, DialogWindow, DialogResult, WindowDrawMode};
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdlvalues::*;
use game::{Game, Animation, Command, DoPlayerAction, InfoGetter};
use game::extrait::*;
use config::UI_CFG;
use draw::border::draw_rect_border;
use eventhandler::InputMode;
use super::widget::*;
use super::misc_window::PageWindow;
use common::gamedata::*;
use game::item::filter::*;

pub type ActionCallback = FnMut(&mut DoPlayerAction, ItemLocation) -> DialogResult;
pub enum ItemWindowMode {
    List, PickUp, Drop, Drink, Eat, ShopSell,
    ShopBuy {
        cid: CharaId,
    },
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
    item_locations: Vec<ItemLocation>,
    page_window: PageWindow,
}

impl ItemWindow {
    pub fn new(mode: ItemWindowMode, pa: &mut DoPlayerAction) -> ItemWindow {
        let rect = UI_CFG.item_window.rect.into();
        
        let mut item_window = ItemWindow {
            rect: rect,
            list: ListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32), ListRowKind::IconStrStr,
                UI_CFG.item_window.column_pos.clone(),
                Some(UI_CFG.item_window.n_row), 26),
            mode: mode,
            item_locations: Vec::new(),
            page_window: PageWindow::new(None, Some(rect.bottom() + UI_CFG.page_window.margin_to_parent)),
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
                let ill = ItemListLocation::Chara { cid: CharaId::Player };
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
            ItemWindowMode::Drop => {
                let ill = ItemListLocation::Chara { cid: CharaId::Player };
                let filtered_list = gd.get_filtered_item_list(ill, ItemFilter::all());
                self.update_list(filtered_list);
            }
            ItemWindowMode::Drink => {
                let ill = ItemListLocation::Chara { cid: CharaId::Player };
                let filtered_list = gd
                    .get_filtered_item_list(ill, ItemFilter::new().flags(ItemFlags::DRINKABLE));
                self.update_list(filtered_list);
            }
            ItemWindowMode::Eat => {
                let ill = ItemListLocation::Chara { cid: CharaId::Player };
                let filtered_list = gd
                    .get_filtered_item_list(ill, ItemFilter::new().flags(ItemFlags::EATABLE));
                self.update_list(filtered_list);
            }
            ItemWindowMode::ShopBuy { cid } => {
                let ill = ItemListLocation::Shop { cid };
                let filtered_list = gd.get_filtered_item_list(ill, ItemFilter::new());
                self.update_list(filtered_list);
            }
            ItemWindowMode::ShopSell => {
                let ill = ItemListLocation::Chara { cid: CharaId::Player };
                let filtered_list = gd.get_filtered_item_list(ill, ItemFilter::new());
                self.update_list(filtered_list);
            }
            ItemWindowMode::Select { ill, filter, ..} => {
                let filtered_list = gd.get_filtered_item_list(ill, filter);
                self.update_list(filtered_list);
            }
        }

        self.page_window.set_page(self.list.get_page(), self.list.get_max_page());
    }

    fn update_list(&mut self, list: FilteredItemList) {
        self.list.set_n_item(list.clone().count() as u32);
        let list = &list;
        
        let item_locations = &mut self.item_locations;
        self.list.update_rows_by_func(|start, page_size| {
            let mut rows = Vec::new();
            item_locations.clear();

            for (item_location, item, n_item) in list.clone().skip(start as usize).take(page_size as usize) {
                let item_text = format!(
                    "{} x {}",
                    item.get_name(),
                    n_item);

                let additional_info = format!("{}kg", item.w() as f32 / 1000.0);
                
                rows.push(ListRow::IconStrStr(IconIdx::Item(item.idx), item_text, additional_info));
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
            ItemWindowMode::Drop => {
                pa.drop_item(il, 1);
                self.update_by_mode(pa);
                DialogResult::Continue
            }
            ItemWindowMode::Drink => {
                pa.drink_item(il);
                self.update_by_mode(pa);
                DialogResult::CloseAll
            }
            ItemWindowMode::Eat => {
                pa.eat_item(il);
                self.update_by_mode(pa);
                DialogResult::CloseAll
            }
            ItemWindowMode::ShopBuy { .. } => {
                pa.buy_item(il);
                self.update_by_mode(pa);
                DialogResult::Continue
            }
            ItemWindowMode::ShopSell => {
                pa.sell_item(il);
                self.update_by_mode(pa);
                DialogResult::Continue
            }
            ItemWindowMode::Select { ref mut action, .. } => {
                action(pa, il)
            }
        }
    }
}

impl Window for ItemWindow {
    
    fn draw(
        &mut self, canvas: &mut WindowCanvas, game: &Game, sv: &mut SdlValues,
        anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);
        self.list.draw(canvas, sv);
        self.page_window.draw(canvas, game, sv, anim);
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
                ListWidgetResponse::PageChanged => {
                    self.update_by_mode(pa);
                }
                _ => (),
            }
            return DialogResult::Continue;
        }
        
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

    fn draw_mode(&self) -> WindowDrawMode {
        WindowDrawMode::SkipUnderWindows
    }
}

