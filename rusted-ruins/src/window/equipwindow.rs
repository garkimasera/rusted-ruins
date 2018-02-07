
use std::any::Any;
use window::{Window, DialogWindow, DialogResult};
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdlvalues::*;
use game::{Game, Animation, Command, DoPlayerAction};
use config::UI_CFG;
use draw::border::draw_rect_border;
use eventhandler::InputMode;
use super::widget::*;
use common::gobj;
use common::gamedata::item::ItemKind;
use common::gamedata::chara::CharaId;
use common::gamedata::item::*;
use text;

pub struct EquipWindow {
    rect: Rect,
    list: ListWidget,
    n_row: u32,
    cid: CharaId,
    slots: Vec<(ItemKind, u8)>
}

impl EquipWindow {
    pub fn new(pa: &mut DoPlayerAction, cid: CharaId) -> EquipWindow {
        let rect = UI_CFG.equip_window.rect.into();
        
        let mut equip_window = EquipWindow {
            rect: rect,
            list: ListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32), ListRowKind::StrIconStr, vec![0, 100, 126],
                Some(UI_CFG.equip_window.n_row), 26),
            n_row: UI_CFG.equip_window.n_row,
            cid: cid,
            slots: Vec::new(),
        };
        equip_window.update_list(pa);
        equip_window
    }

    fn update_list(&mut self, pa: &mut DoPlayerAction) {
        let mut rows = Vec::new();
        let equips = pa.gd().get_equip_list(self.cid);
        self.list.set_n_item(equips.n_slots());
        let slots = &mut self.slots;

        self.list.update_rows_by_func(|start, page_size| {
            slots.clear();
            for (ik, ik_i, item) in equips.slot_iter().skip(start as usize).take(page_size as usize) {
                let kind = text::ui_txt(&format!("{:?}", ik)).to_owned();
                if let Some(item) = item {
                    let item_text = text::obj_txt(&gobj::get_obj(item.idx).id).to_owned();
                    rows.push(ListRow::StrIconStr(kind, IconIdx::Item(item.idx), item_text));
                } else {
                    rows.push(ListRow::StrIconStr(
                        kind,
                        IconIdx::Item(::common::objholder::ItemIdx(0)),
                        text::ui_txt("Empty").to_owned()));
                }
                slots.push((ik, ik_i));
            }
            rows
        });
    }
}

impl Window for EquipWindow {
    
    fn redraw(
        &mut self, canvas: &mut WindowCanvas, _game: &Game, sv: &mut SdlValues,
        _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(canvas, self.rect);
        self.list.draw(canvas, sv);
    }
}

impl DialogWindow for EquipWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        
        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(i) => { // Any item is selected
                    use super::itemwindow::ItemWindow;

                    // Callback function for selected item equipment
                    let slot = self.slots[i as usize];
                    let cid = self.cid;
                    let equip_selected_item = move |pa: &mut DoPlayerAction, il: ItemLocation| {
                        pa.change_equipment(cid, slot, il);
                        DialogResult::Close
                    };
                    
                    let select_window = ItemWindow::new_select(
                        ItemListLocation::Chara { cid: CharaId::Player },
                        ItemFilter::new().kind(self.slots[i as usize].0),
                        Box::new(equip_selected_item),
                        pa
                    );
                    return DialogResult::OpenChildDialog(Box::new(select_window));
                }
                ListWidgetResponse::PageChanged => {
                    self.update_list(pa);
                }
                _ => (),
            }
            return DialogResult::Continue;
        }
        self.list.process_command(&command);
        
        match *command {
            Command::Cancel => {
                DialogResult::Close
            }
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }

    fn callback_child_closed(
        &mut self, _result: Option<Box<Any>>, pa: &mut DoPlayerAction) -> DialogResult {
        
        self.update_list(pa);
        DialogResult::Continue
    }
}

