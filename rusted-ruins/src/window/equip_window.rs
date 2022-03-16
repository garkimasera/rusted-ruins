use super::choose_window::ChooseWindow;
use super::choose_window::DefaultBehavior;
use super::commonuse::*;
use super::item_info_window::ItemInfoWindow;
use super::item_window::ItemWindow;
use super::widget::*;
use crate::draw::border::draw_window_border;
use crate::text::{self, ui_txt};
use crate::window::{DialogResult, DialogWindow, Window};
use common::gamedata::*;
use common::gobj;
use common::objholder::*;
use sdl2::rect::Rect;

pub struct EquipWindow {
    rect: Rect,
    closer: DialogCloser,
    list: ListWidget<(IconIdx, IconIdx, TextCache)>,
    cid: CharaId,
    menu: Option<EquipMenu>,
    changeable: bool,
}

impl EquipWindow {
    pub fn new(gd: &GameData, cid: CharaId, changeable: bool) -> EquipWindow {
        let mut rect: Rect = UI_CFG.info_window.rect.into();
        rect.h = (UI_CFG.equip_window.n_row * UI_CFG.list_widget.h_row_default) as _;

        let mut equip_window = EquipWindow {
            rect,
            closer: DialogCloser::new(rect),
            list: ListWidget::new(
                (0i32, 0i32, rect.w as u32, rect.h as u32),
                UI_CFG.equip_window.column_pos.clone(),
                UI_CFG.equip_window.n_row,
                true,
            ),
            cid,
            menu: None,
            changeable,
        };
        equip_window.update_list(gd);
        equip_window
    }

    fn update_list(&mut self, gd: &GameData) {
        let equips = gd.get_equip_list(self.cid);
        self.list.set_n_item(equips.n_slots());

        self.list.update_rows_by_func(|i| {
            let (esk, _, item) = equips.slot_iter().nth(i as usize).unwrap();
            let esk_icon = slotkind_to_icon_idx(esk);
            if let Some(item) = item {
                let item_text = text::obj_txt(&gobj::get_obj(item.idx).id);
                let tc = TextCache::new(item_text, FontKind::M, UI_CFG.color.normal_font);
                (esk_icon, IconIdx::from(item.idx), tc)
            } else {
                let tc = TextCache::new("-", FontKind::M, UI_CFG.color.normal_font);
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
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        anim: Option<(&Animation, u32)>,
    ) {
        self.closer.draw(context);
        draw_window_border(context, self.rect);
        self.list.draw(context);
        if let Some(menu) = self.menu.as_mut() {
            menu.draw(context, game, anim);
        }
    }
}

impl DialogWindow for EquipWindow {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        if let Some(menu) = self.menu.as_mut() {
            match menu.process_command(command, pa) {
                DialogResult::Special(SpecialDialogResult::ItemListUpdate) => {
                    self.update_list(pa.gd());
                    self.menu = None;
                    return DialogResult::Continue;
                }
                DialogResult::Close => {
                    self.menu = None;
                    return DialogResult::Continue;
                }
                DialogResult::CloseAll => {
                    self.menu = None;
                    return DialogResult::CloseAll;
                }
                DialogResult::OpenChildDialog(child) => {
                    self.menu = None;
                    return DialogResult::OpenChildDialog(child);
                }
                result => {
                    return result;
                }
            }
        }

        let cursor_pos = if let Command::MouseButtonUp { x, y, .. } = command {
            Some((*x, *y))
        } else {
            None
        };

        closer!(self, command);

        let command = command.relative_to(self.rect);

        if let Some(response) = self.list.process_command(&command) {
            match response {
                ListWidgetResponse::Select(i) => {
                    if !self.changeable {
                        return DialogResult::Continue;
                    }

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
                ListWidgetResponse::SelectForMenu(i) => {
                    // Workaround for menu not working in newgame window
                    if !self.changeable && self.cid == CharaId::Player {
                        return DialogResult::Continue;
                    }

                    let equip_list = pa.gd().get_equip_list(self.cid);
                    let (esk, esk_i, _) = equip_list.slot_iter().nth(i as usize).unwrap();
                    if !equip_list.is_slot_empty(esk, esk_i as usize) {
                        self.menu = Some(EquipMenu::new(
                            pa.gd(),
                            self.cid,
                            esk,
                            esk_i,
                            cursor_pos.unwrap(),
                            self.changeable,
                        ));
                    }
                }
                ListWidgetResponse::Scrolled => {
                    self.update_list(pa.gd());
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

    fn callback_child_closed(
        &mut self,
        _result: Option<DialogCloseValue>,
        pa: &mut DoPlayerAction<'_>,
    ) -> DialogResult {
        self.update_list(pa.gd());
        DialogResult::Continue
    }

    fn tab_switched(&mut self) {
        self.menu = None;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EquipMenuItem {
    Remove,
    Information,
}

struct EquipMenu {
    choose_window: ChooseWindow,
    menu_items: Vec<EquipMenuItem>,
    cid: CharaId,
    esk: EquipSlotKind,
    i: u8,
}

impl EquipMenu {
    pub fn new(
        _gd: &GameData,
        cid: CharaId,
        esk: EquipSlotKind,
        i: u8,
        pos: (i32, i32),
        changeable: bool,
    ) -> Self {
        let winpos = WindowPos::from_left_top(pos.0, pos.1);

        let mut choices = Vec::new();
        let mut menu_items = Vec::new();

        if changeable {
            choices.push(ui_txt("equip_menu-remove"));
            menu_items.push(EquipMenuItem::Remove);
        }

        choices.push(ui_txt("equip_menu-information"));
        menu_items.push(EquipMenuItem::Information);

        let choose_window = ChooseWindow::new(winpos, choices, DefaultBehavior::Close);

        EquipMenu {
            choose_window,
            menu_items,
            cid,
            esk,
            i,
        }
    }
}

impl Window for EquipMenu {
    fn draw(
        &mut self,
        context: &mut Context<'_, '_, '_, '_>,
        game: &Game,
        anim: Option<(&Animation, u32)>,
    ) {
        self.choose_window.draw(context, game, anim);
    }
}

impl DialogWindow for EquipMenu {
    fn process_command(&mut self, command: &Command, pa: &mut DoPlayerAction<'_>) -> DialogResult {
        match self.choose_window.process_command(command, pa) {
            DialogResult::CloseWithValue(v) => {
                if let DialogCloseValue::Index(n) = v {
                    let item = self.menu_items[n as usize];
                    let gd = pa.gd();
                    let il = gd
                        .equipment_item_location(self.cid, self.esk, self.i as usize)
                        .unwrap();

                    match item {
                        EquipMenuItem::Remove => {
                            pa.remove_equipment(self.cid, (self.esk, self.i as u8));
                            DialogResult::Special(SpecialDialogResult::ItemListUpdate)
                        }
                        EquipMenuItem::Information => {
                            let info_win = ItemInfoWindow::new(il, pa.game());
                            DialogResult::OpenChildDialog(Box::new(info_win))
                        }
                    }
                } else {
                    unreachable!()
                }
            }
            result => result,
        }
    }
}

fn slotkind_to_icon_idx(esk: EquipSlotKind) -> IconIdx {
    let id = match esk {
        EquipSlotKind::MeleeWeapon => "!icon-melee-weapon",
        EquipSlotKind::RangedWeapon => "!icon-ranged-weapon",
        EquipSlotKind::Tool => "!icon-tool",
        EquipSlotKind::Shield => "!icon-shield",
        EquipSlotKind::Head => "!icon-head",
        EquipSlotKind::Skin => "!icon-skin",
        EquipSlotKind::Body => "!icon-body",
        EquipSlotKind::Arms => "!icon-arms",
        EquipSlotKind::Legs => "!icon-legs",
        EquipSlotKind::ArmorPlate => "!icon-armor_plate",
        EquipSlotKind::Accessory => "!icon-accessory",
    };
    let idx: UiImgIdx = gobj::id_to_idx(id);
    IconIdx::from(idx)
}
