use super::widget::*;
use crate::config::UI_CFG;
use crate::context::*;
use crate::draw::border::draw_rect_border;
use crate::eventhandler::InputMode;
use crate::game::item::filter::*;
use crate::game::{Animation, Command, DoPlayerAction, Game};
use crate::text;
use crate::window::{DialogResult, DialogWindow, Window};
use common::gamedata::*;
use common::gobj;
use sdl2::rect::Rect;
use std::any::Any;

pub struct EquipWindow {
    rect: Rect,
    list: ListWidget<(IconIdx, IconIdx, TextCache)>,
    cid: CharaId,
}

impl EquipWindow {
    pub fn new(pa: &mut DoPlayerAction, cid: CharaId) -> EquipWindow {
        let rect = UI_CFG.equip_window.rect.into();

        let mut equip_window = EquipWindow {
            rect,
            list: ListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32),
                UI_CFG.equip_window.column_pos.clone(),
                UI_CFG.equip_window.n_row,
                true,
            ),
            cid,
        };
        equip_window.update_list(pa);
        equip_window
    }

    fn update_list(&mut self, pa: &mut DoPlayerAction) {
        let equips = pa.gd().get_equip_list(self.cid);
        self.list.set_n_item(equips.n_slots());

        self.list.update_rows_by_func(|i| {
            let (esk, _, item) = equips.slot_iter().nth(i as usize).unwrap();
            let esk_icon = slotkind_to_icon_idx(esk);
            if let Some(item) = item {
                let item_text = text::obj_txt(&gobj::get_obj(item.idx).id).to_owned();
                let tc = TextCache::one(item_text, FontKind::M, UI_CFG.color.normal_font.into());
                (esk_icon, IconIdx::Item(item.idx), tc)
            } else {
                let tc = TextCache::one("-", FontKind::M, UI_CFG.color.normal_font.into());
                (
                    esk_icon,
                    IconIdx::Item(common::objholder::ItemIdx::default()),
                    tc,
                )
            }
        });
    }
}

impl Window for EquipWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_rect_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for EquipWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(i) => {
                    // Any item is selected
                    use super::item_window::ItemWindow;

                    // Callback function for selected item equipment
                    let cid = self.cid;
                    let (esk, esk_i, _) = pa
                        .gd()
                        .get_equip_list(cid)
                        .slot_iter()
                        .nth(i as usize)
                        .unwrap();
                    let slot = (esk, esk_i);
                    let equip_selected_item = move |pa: &mut DoPlayerAction, il: ItemLocation| {
                        pa.change_equipment(cid, slot, il);
                        DialogResult::Close
                    };

                    let select_window = ItemWindow::new_select(
                        ItemListLocation::Chara {
                            cid: CharaId::Player,
                        },
                        ItemFilter::new().equip_slot_kind(slot.0),
                        Box::new(equip_selected_item),
                        pa,
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
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }

    fn callback_child_closed(
        &mut self,
        _result: Option<Box<dyn Any>>,
        pa: &mut DoPlayerAction,
    ) -> DialogResult {
        self.update_list(pa);
        DialogResult::Continue
    }
}

fn slotkind_to_icon_idx(esk: EquipSlotKind) -> IconIdx {
    let id = match esk {
        EquipSlotKind::MeleeWeapon => "!icon-melee-weapon",
        EquipSlotKind::RangedWeapon => "!icon-ranged-weapon",
        EquipSlotKind::BodyArmor => "!icon-bodyarmor",
        EquipSlotKind::Shield => "!icon-shield",
    };
    IconIdx::UIImg(gobj::id_to_idx(id))
}
