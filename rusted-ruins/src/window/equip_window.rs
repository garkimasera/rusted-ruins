
use std::any::Any;
use crate::window::{Window, DialogWindow, DialogResult};
use sdl2::rect::Rect;
use crate::context::*;
use crate::game::{Game, Animation, Command, DoPlayerAction};
use crate::config::UI_CFG;
use crate::draw::border::draw_rect_border;
use crate::eventhandler::InputMode;
use super::widget::*;
use common::gobj;
use common::gamedata::*;
use crate::game::item::filter::*;
use crate::text;

pub struct EquipWindow {
    rect: Rect,
    list: ListWidget,
    cid: CharaId,
    slots: Vec<(EquipSlotKind, u8)>
}

impl EquipWindow {
    pub fn new(pa: &mut DoPlayerAction, cid: CharaId) -> EquipWindow {
        let rect = UI_CFG.equip_window.rect.into();
        
        let mut equip_window = EquipWindow {
            rect: rect,
            list: ListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32), ListRowKind::IconIconStr,
                UI_CFG.equip_window.column_pos.clone(),
                Some(UI_CFG.equip_window.n_row), 26),
            cid: cid,
            slots: Vec::new(),
        };
        equip_window.update_list(pa);
        equip_window
    }

    fn update_list(&mut self, pa: &mut DoPlayerAction) {
        let equips = pa.gd().get_equip_list(self.cid);
        self.list.set_n_item(equips.n_slots());
        let slots = &mut self.slots;

        self.list.update_rows_by_func(|start, page_size| {
            let mut rows = Vec::new();
            slots.clear();
            for (esk, esk_i, item) in equips.slot_iter().skip(start as usize).take(page_size as usize) {
                let esk_icon = slotkind_to_icon_idx(esk);
                if let Some(item) = item {
                    let item_text = text::obj_txt(&gobj::get_obj(item.idx).id).to_owned();
                    rows.push(ListRow::IconIconStr(esk_icon, IconIdx::Item(item.idx), item_text));
                } else {
                    rows.push(ListRow::IconIconStr(
                        esk_icon,
                        IconIdx::Item(common::objholder::ItemIdx::default()),
                        "-".to_owned()));
                }
                slots.push((esk, esk_i));
            }
            rows
        });
    }
}

impl Window for EquipWindow {
    
    fn draw(
        &mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        
        draw_rect_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for EquipWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        
        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(i) => { // Any item is selected
                    use super::item_window::ItemWindow;

                    // Callback function for selected item equipment
                    let slot = self.slots[i as usize];
                    let cid = self.cid;
                    let equip_selected_item = move |pa: &mut DoPlayerAction, il: ItemLocation| {
                        pa.change_equipment(cid, slot, il);
                        DialogResult::Close
                    };
                    
                    let select_window = ItemWindow::new_select(
                        ItemListLocation::Chara { cid: CharaId::Player },
                        ItemFilter::new().equip_slot_kind(self.slots[i as usize].0),
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
        &mut self, _result: Option<Box<dyn Any>>, pa: &mut DoPlayerAction) -> DialogResult {
        
        self.update_list(pa);
        DialogResult::Continue
    }
}

fn slotkind_to_icon_idx(esk: EquipSlotKind) -> IconIdx {
    let id = match esk {
        EquipSlotKind::MeleeWeapon  => "!icon-melee-weapon",
        EquipSlotKind::RangedWeapon => "!icon-ranged-weapon",
        EquipSlotKind::BodyArmor    => "!icon-bodyarmor",
        EquipSlotKind::Shield       => "!icon-shield",
    };
    IconIdx::UIImg(gobj::id_to_idx(id))
}

