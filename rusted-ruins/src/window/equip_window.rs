use super::commonuse::*;
use super::widget::*;
use crate::draw::border::draw_window_border;
use crate::eventhandler::InputMode;
use crate::text;
use crate::window::{DialogResult, DialogWindow, Window};
use common::gamedata::*;
use common::gobj;
use common::objholder::*;
use sdl2::rect::Rect;

pub struct EquipWindow {
    rect: Rect,
    list: ListWidget<(IconIdx, IconIdx, TextCache)>,
    cid: CharaId,
    escape_click: bool,
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
            escape_click: false,
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
                (esk_icon, IconIdx::from(item.idx), tc)
            } else {
                let tc = TextCache::one("-", FontKind::M, UI_CFG.color.normal_font.into());
                (
                    esk_icon,
                    IconIdx::from(common::objholder::ItemIdx::default()),
                    tc,
                )
            }
        });
    }
}

impl Window for EquipWindow {
    fn draw(&mut self, context: &mut Context, _game: &Game, _anim: Option<(&Animation, u32)>) {
        draw_window_border(context, self.rect);
        self.list.draw(context);
    }
}

impl DialogWindow for EquipWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction) -> DialogResult {
        check_escape_click!(self, command);

        let command = command.relative_to(self.rect);

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

                    let select_window = ItemWindow::new_select_and_equip(cid, slot, pa);
                    return DialogResult::OpenChildDialog(Box::new(select_window));
                }
                ListWidgetResponse::Scrolled => {
                    self.update_list(pa);
                }
                _ => (),
            }
            return DialogResult::Continue;
        }

        match command {
            Command::Cancel => DialogResult::Close,
            _ => DialogResult::Continue,
        }
    }

    fn mode(&self) -> InputMode {
        InputMode::Dialog
    }

    fn callback_child_closed(
        &mut self,
        _result: Option<DialogCloseValue>,
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
        EquipSlotKind::Tool => "!icon-tool",
        EquipSlotKind::BodyArmor => "!icon-bodyarmor",
        EquipSlotKind::Shield => "!icon-shield",
    };
    let idx: UIImgIdx = gobj::id_to_idx(id);
    IconIdx::from(idx)
}
